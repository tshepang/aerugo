"""Module with environment-related constants and functions used by Aerugo/Calldwell.
It contains all information and functions necessary to build and execute test/example
binaries on target hardware.

By default, Aerugo/Calldwell supports debugging and running the binaries on development
setup accessible remotely via SSH. Credentials and network location of development setup
is fetched from environmental variables, which should be set by the developer. See
`Environmental variables` section of this module for more details.

Additionally, it provides some useful project-related constants.
"""

import os
from pathlib import Path

# Board credentials
BOARD_LOGIN = str(os.environ.get("AERUGO_BOARD_LOGIN"))
"""SSH login of development setup"""
BOARD_PASSWORD = str(os.environ.get("AERUGO_BOARD_PASSWORD"))
"""SSH password of development setup.
Key-based authorization is currently unsupported."""
BOARD_NETWORK_PATH = str(os.environ.get("AERUGO_BOARD_HOSTNAME"))
"""Network path of development setup, might be a domain name or IP address"""

# Board environment
BOARD_DEBUGGING_SCRIPT_PATH = "./run_openocd_samv71_clean.sh"
"""This script should run GDB server bound to a TCP port provided in `BOARD_GDB_PORT`
constant"""
BOARD_GDB_PORT = int(str(os.environ.get("AERUGO_BOARD_GDB_PORT")))
"""GDB TCP port of development setup"""
BOARD_RTT_PORT = int(str(os.environ.get("AERUGO_BOARD_RTT_PORT")))
"""RTT TCP port of development setup"""
BOARD_TARGET_TRIPLE = "thumbv7em-none-eabihf"
"""Target triple of development setup"""

# Host environment
HOST_GDB_EXECUTABLE = "arm-none-eabi-gdb"
"""GDB executable used by the host to connect to remote GDB session."""

# Aerugo constants. All constants are relative to Aerugo's root directory.
INTEGRATION_TESTS_DIRECTORY = Path("./testbins")
"""Directory with integration tests projects"""
EXAMPLES_DIRECTORY = Path("./examples")
"""Directory with example projects"""
LOGGER_INIT_FUNCTION_NAME = "aerugo_cortex_m::logger::init_log"
"""Function performing RTT logger initialization"""
