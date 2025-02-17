"""Module containing low-level GDB types and functions.
These are usually managed by high-level counterparts, like GDBClient
from `gdb_client` module. Don't use, unless you know what you're doing."""

from __future__ import annotations

import logging
import signal
from dataclasses import dataclass
from typing import TYPE_CHECKING, Any

from pygdbmi.gdbcontroller import GdbController

from .gdb_responses import GDBResponse, GDBResponsesList

if TYPE_CHECKING:
    from collections.abc import Iterable


class GDBInterface:
    """Class for controlling GDB and performing low-level GDB operations.
    Manages GDB instance, performs preliminary GDB message parsing, and provides
    functional interface for common GDB actions.
    Also tracks notifications and watches the state of debugged program.

    Names prefixed with '_' are private, and should never be used in user code.
    """

    def __init__(
        self: GDBInterface,
        gdb_executable: str,
        default_timeout: float,
        log_execution: bool = True,
        log_responses: bool = True,
    ) -> None:
        """Initialize a GDB client.

        # Parameters
        * `gdb_exec` - GDB executable (name or path, will be resolved by OS).
        * `default_timeout` - Default timeout, in seconds, for GDB commands.
        * `log_execution` - If `True`, all executed commands will be logged.
        * `log_responses` - If `True`, all responses to commands will be logged.
        """
        self._logger = logging.getLogger(self.__class__.__name__)
        self._controller = GdbController(command=[gdb_executable, "--interpreter=mi2"])
        self._default_timeout = default_timeout
        self._should_log_execution = log_execution
        self._should_log_responses = log_responses
        self._program_state = ProgramState()

        self._logger.info(
            f"GDB interface created for '{gdb_executable}' with default command timeout "
            f"of {default_timeout}s, received following startup message:\n"
            + self._parse_init_message(),
        )

        self.execute(
            f"set remotetimeout {int(self._default_timeout)}",
            self._default_timeout,
            self._should_log_execution,
        )
        self.wait_for_done(self._default_timeout, self._should_log_responses)

    @property
    def program_state(self: GDBInterface) -> ProgramState:
        """Returns the state of currently debugged program. State is tracked via notifications."""
        return self._program_state

    def interrupt(self: GDBInterface) -> None:
        """Send SIGINT to GDB process, in order to interrupt it
        (and, for example, pause execution)"""
        self._controller.gdb_process.send_signal(signal.SIGINT)  # type: ignore

    def execute(
        self: GDBInterface,
        command: str,
        timeout: float | None = None,
        log_execution: bool | None = None,
    ) -> None:
        """Executes a GDB command. Does not fetch any response, use `get_responses` to do it
        manually, or one of `execute_and_*` functions to block until an expected response is
        received.

        # Parameters
        * `command` - GDB/MI command to execute.
        * `timeout` - Timeout for this command, overrides default one if provided.
        * `log_execution` - If provided, will override `self.log_execution`.
        """
        if timeout is None:
            timeout = self._default_timeout

        if log_execution is None:
            log_execution = self._should_log_execution

        if self._should_log_execution:
            self._logger.info(f"Executing '{command}' (timeout = {timeout}s)")

        self._controller.write(  # type: ignore
            command,
            timeout_sec=timeout,
            raise_error_on_timeout=True,
            read_response=False,
        )

    def get_responses(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Fetches all available responses from GDB.
        This is a wrap, similarly to `execute`, mostly for type safety.
        Also tracks the state of GDB and the program via notifications.
        DO NOT USE `self.controller` TO FETCH GDB RESPONSES DIRECTLY IN USER CODE, OTHERWISE
        STATE TRACKING WILL NOT WORK CORRECTLY.

        # Parameters
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        if timeout is None:
            timeout = self._default_timeout

        if log_responses is None:
            log_responses = self._should_log_responses

        raw_responses: list[dict[str, Any]] = self._controller.get_gdb_response(  # type: ignore
            timeout,
        )

        responses = GDBResponsesList(
            [
                GDBResponse(
                    message=response.get("message"),
                    payload=response.get("payload"),
                    token=response.get("token"),
                    response_type=GDBResponse.Type(response.get("type")),
                    stream=GDBResponse.Stream(response.get("stream")),
                )
                for response in raw_responses
            ],
        )

        if log_responses:
            self._log_responses(responses)

        self._handle_async_events(responses)

        return responses

    def wait_for_response(
        self: GDBInterface,
        expected_response: GDBResponse,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Block until an expected response is received from GDB.

        # Remarks
        Responses are compared using `GDBResponse.is_similar()` function.

        # Parameters
        * `expected_response` - Response to wait for.
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        received_responses = self.get_responses(timeout, log_responses)

        while expected_response not in received_responses:
            received_responses.extend(self.get_responses(timeout, log_responses))

        return received_responses

    def wait_for_any_response(
        self: GDBInterface,
        expected_responses: Iterable[GDBResponse],
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Block until any response from the list is received from GDB.

        # Remarks
        Responses are compared using `GDBResponse.is_similar()` function.

        # Parameters
        * `expected_responses` - Responses to wait for. Receiving a response matching any that's
                                 on the list will unblock.
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        received_responses = self.get_responses(timeout, log_responses)

        while not received_responses.contains_any(expected_responses):
            received_responses.extend(self.get_responses(timeout, log_responses))

        return received_responses

    def wait_for_all_responses(
        self: GDBInterface,
        expected_responses: Iterable[GDBResponse],
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Block until all responses from the list are received from GDB.

        # Remarks
        Responses are compared using `GDBResponse.is_similar()` function.

        # Parameters
        * `expected_responses` - Responses to wait for. All responses from this list must match
                                 received responses to unblock.
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        received_responses = self.get_responses(timeout, log_responses)

        while not received_responses.contains_all(expected_responses):
            received_responses.extend(self.get_responses(timeout, log_responses))

        return received_responses

    def wait_for_result(
        self: GDBInterface,
        message: str,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `result` response with specified message is received.
        Returns list of all received responses.

        # Parameters
        * `message` - Message to wait for.
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        return self.wait_for_response(
            GDBResponse.with_message(GDBResponse.Type.RESULT, message),
            timeout,
            log_responses,
        )

    def wait_for_any_result(
        self: GDBInterface,
        messages: Iterable[str],
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `result` response with message matching any of the provided ones
        is received.
        Returns list of all received responses.

        # Parameters
        * `messages` - List of messages to wait for. Matching any of them will unblock.
        * `timeout` - If provided, overrides default timeout.
        * `log_responses` - If provided, will override `self.log_responses`.
        """
        expected_responses = [
            GDBResponse.with_message(GDBResponse.Type.RESULT, message) for message in messages
        ]
        return self.wait_for_any_response(expected_responses, timeout, log_responses)

    def wait_for_notification(
        self: GDBInterface,
        message: str,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `notify` response with specified message is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_response(
            GDBResponse.with_message(GDBResponse.Type.NOTIFY, message),
            timeout,
            log_responses,
        )

    def wait_for_done(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `{type: result, message: done}` response is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_result("done", timeout, log_responses)

    def wait_for_error(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `{type: result, message: error}` response is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_result("error", timeout, log_responses)

    def wait_for_done_or_error(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `{type: result, message: done}` or `{type: result, message: error}'
        response is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_any_result(["done", "error"], timeout, log_responses)

    def wait_for_running(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `{type: result, message: running}` response is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_result("running", timeout, log_responses)

    def wait_for_stopped(
        self: GDBInterface,
        timeout: float | None = None,
        log_responses: bool | None = None,
    ) -> GDBResponsesList:
        """Blocks until a `{type: notify, message: stopped}` response is received.
        Returns list of all received responses.
        See `wait_for_response` for details."""
        return self.wait_for_notification("stopped", timeout, log_responses)

    def _parse_init_message(self: GDBInterface) -> str:
        """Private function. Do not use.
        Parses init message received from GDB after creating it's instance, and returns it
        as a string"""
        return self.get_responses(log_responses=False).console().payload_string()

    def _log_responses(
        self: GDBInterface,
        responses: GDBResponsesList,
    ) -> None:
        """Private function. Do not use.
        Logs the response in appropriate way, depending on it's type.

        # Remarks
        Will do nothing if logging GDB responses is disabled.

        # Parameters
        * `responses` - List of responses to be logged.
        * `separator` - Character (or a string) inserted between each response's payload.
        """
        if not self._should_log_responses:
            return

        for response in responses:
            log_message: str = f"[Response <{response.response_type.value}>]:"
            if response.response_type == GDBResponse.Type.NOTIFY:
                log_message += f" {response.message} -> {response.unescaped_payload().strip()}"
            elif response.response_type == GDBResponse.Type.RESULT:
                log_message += f" {response.message}"
                if response.payload_is_json():
                    additional_message = response.payload_json().get(
                        "msg",
                        response.unescaped_payload().strip(),
                    )
                    log_message += f" ({additional_message})"
            else:
                if response.message is not None:
                    log_message += f" [msg: {response.message}]"
                if response.payload is not None:
                    log_message += f" {response.unescaped_payload().strip()}"
            self._logger.info(log_message)

    def _handle_async_events(self: GDBInterface, responses: GDBResponsesList) -> None:
        """Private function. Do not use.
        Looks through responses on the list and changes this object's state according to received
        messages."""
        for notification in responses.notifications():
            if notification.message == "stopped":
                self._program_state.is_running = False

                payload = notification.payload_json()
                current_program_frame = ProgramFrame(
                    address=int(payload["frame"]["addr"], 16),
                    function=payload["frame"]["func"],
                )
                stop_reason = payload.get("reason", None)

                self._program_state.last_stop_reason = stop_reason
                self._program_state.program_frame = current_program_frame

                if stop_reason == "signal-received":
                    # temporary fix for not detecting watchdog reset
                    self.program_state.was_reset = True

                self._logger.info(
                    f"Program has stopped at {current_program_frame}, reason: {stop_reason}",
                )
            elif notification.message == "running":
                self._program_state.is_running = True
                self._logger.info("Program is running.")

        for message in responses.target():
            if "external reset detected" in message.unescaped_payload():
                self._program_state.was_reset = True
                self._logger.info("External reset detected!")


@dataclass
class ProgramFrame:
    """Structure representing a current program frame (where Program Counter currently points)"""

    address: int
    function: str

    def address_string(self: ProgramFrame, address_size: int = 4) -> str:
        """Returns address as hex string. Assumes 32-bit addresses by default."""
        stringified_address = f"{self.address:X}".zfill(address_size * 2)
        return f"0x{stringified_address}"

    def __str__(self: ProgramFrame) -> str:
        return f"{self.function} @ {self.address_string()}"


@dataclass
class ProgramState:
    """Container for everything related to program state."""

    is_running: bool = False
    last_stop_reason: str | None = None
    program_frame: ProgramFrame | None = None
    """Program frame is updated and valid only when program is stopped."""
    was_reset: bool = False

    def stopped_by(self: ProgramState, reason: str) -> bool:
        """Returns `True` if program is currently stopped by specified reason."""
        return self.is_running is False and self.last_stop_reason == reason

    def stopped_by_breakpoint(self: ProgramState) -> bool:
        """Shorthand for `is_stopped_by("breakpoint-hit")"""
        return self.stopped_by("breakpoint-hit")

    def function_finished_execution(self: ProgramState) -> bool:
        """Shorthand for `is_stopped_by("function-finished")"""
        return self.stopped_by("function-finished")
