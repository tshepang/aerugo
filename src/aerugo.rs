//! System control and interface for the user to interact with it.
//!
//! This module contains mainly system API that can be used by the user to interact with the
//! system and internal API used by other parts of the system.
//!
//! This module also contains singleton instances of all system parts.

use aerugo_hal::{AerugoHal, SystemHardwareConfig};
use critical_section::CriticalSection;
use env_parser::read_env;

use crate::api::{InitApi, InitError, RuntimeApi, RuntimeError};
#[cfg(feature = "log")]
use crate::arch::init_log;
use crate::boolean_condition::{
    BooleanConditionHandle, BooleanConditionSet, BooleanConditionStorage,
};
use crate::cyclic_execution_manager::CyclicExecutionManager;
use crate::event::{EventId, EventStorage};
use crate::event_manager::EventManager;
use crate::execution_monitoring::ExecutionStats;
use crate::executor::Executor;
use crate::hal::{Hal, UserPeripherals};
use crate::message_queue::{MessageQueueHandle, MessageQueueStorage};
use crate::tasklet::{StepFn, TaskletConfig, TaskletHandle, TaskletId, TaskletPtr, TaskletStorage};
use crate::time_source::TimeSource;
use crate::{Duration, Instant};

/// Core system.
///
/// This is used to access the system API, both by the user and by the internal system parts.
static AERUGO: Aerugo = Aerugo::new();

/// System scheduler.
///
/// Singleton instance of the scheduler. Used directly only by the [Aerugo]
/// structure, which exposes some functionality via it's API.
static EXECUTOR: Executor = Executor::new(AERUGO.time_source());
/// Event manager.
///
/// Singleton instance of the event manager. Used directly only by the [Aerugo]
/// structure.
static EVENT_MANAGER: EventManager = EventManager::new();
/// Time manager.
///
/// Singleton instance of the time manager. Used directly only by the [Aerugo]
/// structure.
static CYCLIC_EXECUTION_MANAGER: CyclicExecutionManager = CyclicExecutionManager::new();

/// System structure.
///
/// This shouldn't be created by hand by the user or anywhere else in the code.
pub struct Aerugo {
    /// Time source, responsible for creating timestamps.
    time_source: TimeSource,
}

impl Aerugo {
    /// Maximum number of tasklets registered in the system.
    #[read_env("AERUGO_TASKLET_COUNT")]
    pub(crate) const TASKLET_COUNT: usize = 0;

    /// Creates new system instance.
    ///
    /// # Safety
    /// This shouldn't be called in more that [one place](crate::aerugo::AERUGO).
    const fn new() -> Self {
        Aerugo {
            time_source: TimeSource::new(),
        }
    }

    /// Returns reference to the system time source.
    pub(crate) const fn time_source(&'static self) -> &'static TimeSource {
        &self.time_source
    }

    /// Initialize the system runtime and hardware.
    pub fn initialize(config: SystemHardwareConfig) -> (&'static impl InitApi, UserPeripherals) {
        #[cfg(feature = "log")]
        init_log();

        Hal::configure_hardware(config)
            .expect("HAL initialization or hardware configuration failed");
        let user_peripherals =
            Hal::create_user_peripherals().expect("Cannot create user peripherals instance");

        (&AERUGO, user_peripherals)
    }

    /// Wakes given tasklet by scheduling it for execution.
    ///
    /// # Parameters
    /// * `tasklet` - Tasklet to wake
    pub(crate) fn wake_tasklet(tasklet: &TaskletPtr) {
        EXECUTOR
            .schedule_tasklet(tasklet)
            .expect("Unable to schedule tasklet");
    }

    /// Runs the system.
    ///
    /// The system works in a loop. On each pass one tasklet is executed and then system updates
    /// its internal components and hardware.
    fn run(&'static self) -> ! {
        loop {
            EXECUTOR
                .execute_next_tasklet()
                .expect("Failure in tasklet execution");

            Hal::feed_watchdog();

            CYCLIC_EXECUTION_MANAGER.wake_tasklets();
        }
    }
}

impl InitApi for Aerugo {
    /// Creates new tasklet in the system.
    ///
    /// Tasklet is created in the passed `storage` memory. Storage has to be static to keep the stored
    /// tasklet for the whole duration of systems' life.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage};
    /// #[derive(Default)]
    /// struct TaskCtx;
    ///
    /// fn task(_: u8, _: &mut TaskCtx, _: &dyn RuntimeApi) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx, 0> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     #
    ///     # assert!(!TASK_STORAGE.is_initialized());
    ///     #
    ///     let task_config = TaskletConfig::default();
    ///     aerugo.create_tasklet(task_config, task, &TASK_STORAGE).expect("Unable to create Tasklet");
    ///     #
    ///     # assert!(TASK_STORAGE.is_initialized());
    ///
    ///     // Do something with the tasklet via handle.
    ///     let task_handle = TASK_STORAGE.create_handle();
    ///     #
    ///     # assert!(task_handle.is_some())
    /// }
    /// ```
    ///
    /// # Notes
    /// ## Given storage cannot be used to create more than one tasklet.
    ///
    /// ```
    /// # use aerugo::{Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage};
    /// #[derive(Default)]
    /// struct TaskCtx;
    ///
    /// fn task(_: u8, _: &mut TaskCtx, _: &dyn RuntimeApi) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx, 0> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     #
    ///     let task_config = TaskletConfig::default();
    ///
    ///     // First call is successful.
    ///     assert!(aerugo.create_tasklet(task_config.clone(), task, &TASK_STORAGE).is_ok());
    ///
    ///     // But second is not.
    ///     assert!(aerugo.create_tasklet(task_config.clone(), task, &TASK_STORAGE).is_err());
    /// }
    /// ```
    ///
    /// ## Storage has to be initialized to create a handle to tasklet.
    /// ```
    /// # use aerugo::{InitApi, RuntimeApi, TaskletConfig, TaskletStorage};
    /// #[derive(Default)]
    /// struct TaskCtx;
    ///
    /// fn task(_: u8, _: &mut TaskCtx, _: &dyn RuntimeApi) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx, 0> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     let task_config = TaskletConfig::default();
    ///
    ///     assert!(TASK_STORAGE.create_handle().is_none());
    /// }
    /// ```
    fn create_tasklet<T, C: Default, const COND_COUNT: usize>(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), InitError> {
        // SAFETY: This is safe, as long as this function is called only during system initialization.
        unsafe { storage.init(config, step_fn, C::default(), self) }
    }

    /// Creates new tasklet in the system with initialized context data.
    ///
    /// Tasklet is created in the passed `storage` memory. Storage has to be static to keep the stored
    /// tasklet for the whole duration of system life.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data processed by the tasklet.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `config` - Tasklet creation configuration.
    /// * `step_fn` - Tasklet step function.
    /// * `context` - Tasklet context data.
    /// * `storage` - Static memory storage where the tasklet should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage};
    /// struct TaskCtx {
    ///     value: u8,
    /// }
    ///
    /// fn task(_: u8, _: &mut TaskCtx, _: &dyn RuntimeApi) {}
    ///
    /// static TASK_STORAGE: TaskletStorage<u8, TaskCtx, 0> = TaskletStorage::new();
    ///
    /// fn main() {
    ///     let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     #
    ///     let task_config = TaskletConfig::default();
    ///     let task_context = TaskCtx { value: 42 };
    ///     aerugo.create_tasklet_with_context(task_config, task, task_context, &TASK_STORAGE);
    ///     #
    ///     # assert!(TASK_STORAGE.is_initialized());
    ///
    ///     // Do something with the tasklet via handle.
    ///     let task_handle = TASK_STORAGE.create_handle();
    ///     #
    ///     # assert!(task_handle.is_some())
    /// }
    /// ```
    fn create_tasklet_with_context<T, C, const COND_COUNT: usize>(
        &'static self,
        config: TaskletConfig,
        step_fn: StepFn<T, C>,
        context: C,
        storage: &'static TaskletStorage<T, C, COND_COUNT>,
    ) -> Result<(), InitError> {
        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe { storage.init(config, step_fn, context, self) }
    }

    /// Creates new message queue in the system.
    ///
    /// Queue is created in the passed `storage` memory. Storage has to be static to keep the
    /// stored tasklet for the whole duration of system life.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data stored in the queue.
    /// * `QUEUE_SIZE` - Size of the queue.
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the queue should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, MessageQueueStorage, SystemHardwareConfig};
    /// static QUEUE_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();
    ///
    /// fn main() {
    ///     let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     #
    ///     # assert!(!QUEUE_STORAGE.is_initialized());
    ///     #
    ///     aerugo.create_message_queue(&QUEUE_STORAGE);
    ///     #
    ///     # assert!(QUEUE_STORAGE.is_initialized());
    ///
    ///     // Do something with the queue via handle.
    ///     let queue_handle = QUEUE_STORAGE.create_handle();
    ///     #
    ///     # assert!(queue_handle.is_some())
    /// }
    /// ```
    fn create_message_queue<T, const QUEUE_SIZE: usize>(
        &'static self,
        storage: &'static MessageQueueStorage<T, QUEUE_SIZE>,
    ) -> Result<(), InitError> {
        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe { storage.init() }
    }

    /// Creates new event in the system.
    ///
    /// Events are created in EventManager and are identifier by the user-provided ID value. ID has
    /// to be unique across the events in the system.
    ///
    /// # Parameters
    /// * `event_id` - ID of the new event.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it modifies event
    /// list which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, EventId, EventStorage, InitApi, SystemHardwareConfig};
    ///
    /// enum Events {
    ///     MyEvent,
    /// }
    ///
    /// impl From<Events> for EventId {
    ///     fn from(value: Events) -> Self {
    ///         match value {
    ///             Events::MyEvent => 42,
    ///         }
    ///     }
    /// }
    ///
    /// static MY_EVENT_STORAGE: EventStorage = EventStorage::new();
    ///
    /// fn main() {
    ///     let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///
    ///     aerugo.create_event(Events::MyEvent.into(), &MY_EVENT_STORAGE);
    /// }
    /// ```
    fn create_event(
        &'static self,
        event_id: EventId,
        storage: &'static EventStorage,
    ) -> Result<(), InitError> {
        if EVENT_MANAGER.has_event(event_id) {
            return Err(InitError::EventAlreadyExists(event_id));
        }

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            storage.init(event_id)?;
        }

        let event = storage
            .event()
            .expect("Failed to get reference to the stored event");
        // SAFETY: This is safe as long as this function is called only during system
        // initialization.
        unsafe {
            EVENT_MANAGER.add_event(event)?;
        }

        Ok(())
    }

    /// Creates new boolean condition in the system.
    ///
    /// Condition is created in the passed `storage` memory. Storage has to be static to keep the
    /// stored condition for the whole duration of system life
    ///
    /// # Parameters
    /// * `storage` - Static memory storage where the condition should be allocated.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because it initializes the
    /// passed storage which is safe only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, BooleanConditionStorage, SystemHardwareConfig};
    /// static CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
    ///
    /// fn main() {
    ///     let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     #
    ///     # assert!(!CONDITION_STORAGE.is_initialized());
    ///     #
    ///     aerugo.create_boolean_condition(&CONDITION_STORAGE, true);
    ///     #
    ///     # assert!(CONDITION_STORAGE.is_initialized());
    ///
    ///     // Do something with the condition via handle.
    ///     let condition_handle = CONDITION_STORAGE.create_handle();
    ///     #
    ///     # assert!(condition_handle.is_some())
    /// }
    /// ```
    fn create_boolean_condition(
        &'static self,
        storage: &'static BooleanConditionStorage,
        value: bool,
    ) -> Result<(), InitError> {
        unsafe { storage.init(value) }
    }

    /// Subscribes a tasklet to a queue.
    ///
    /// Tasklet subscribes for a new data in this queue. Adding new data to the queue will wake up all
    /// subscribed tasklets and make them ready to be executed. Tasklet is ready for an execution for as
    /// long as there is some data in the queue. On each execution tasklet takes one element from
    /// the queue and performs user-specified processing on it.
    ///
    /// Each tasklet can be subscribed to at maximum one data provider. Queue can have multiple
    /// tasklets registered.
    ///
    /// Strong typing is enforced, tasklet can only be subscribed to a queue that stores the same
    /// type of data, that is processed by the tasklet.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    /// * `QUEUE_SIZE` - Size of the queue.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, MessageQueueStorage, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    /// #   TaskletStorage};
    /// #
    /// # fn task(_: u8, _: &mut (), _: &dyn RuntimeApi) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<u8, (), 0> = TaskletStorage::new();
    /// # static QUEUE_STORAGE: MessageQueueStorage<u8, 10> = MessageQueueStorage::new();
    /// #
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     # let task_config = TaskletConfig::default();
    ///     # aerugo
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     # aerugo
    ///     #   .create_message_queue(&QUEUE_STORAGE)
    ///     #   .expect("Unable to create MessageQueue");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///     let queue_handle = QUEUE_STORAGE.create_handle().expect("Failed to create Queue handle");
    ///
    ///     aerugo
    ///         .subscribe_tasklet_to_queue(&task_handle, &queue_handle)
    ///         .expect("Failed to subscribe Task to Queue");
    /// }
    /// ```
    fn subscribe_tasklet_to_queue<
        T: Default,
        C,
        const COND_COUNT: usize,
        const QUEUE_SIZE: usize,
    >(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        queue_handle: &MessageQueueHandle<T, QUEUE_SIZE>,
    ) -> Result<(), InitError> {
        let tasklet = tasklet_handle.tasklet();
        let queue = queue_handle.queue();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            tasklet.subscribe(queue)?;
            queue.register_tasklet(tasklet.ptr())?;
        }

        Ok(())
    }

    /// Subscribes a tasklet to events.
    ///
    /// Tasklet subscribes for emitted events. Emitting an event will wake up all tasklet for which it is enabled
    /// and make them ready to be executed. Tasklet is ready for an execution for as long as there is unhandled
    /// event. On each execution tasklet will handle one event, receiving it's ID in step function.
    ///
    /// Each tasklet can be subscribed to at maximum one data provider. Each event can be active
    /// for multiple tasklets.
    ///
    /// Strong typing is enforced, tasklet can only be subscribed to events if it processes
    /// [EventId] type.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `queue` - Handle to the target queue.
    ///
    /// # Return
    /// `EventEnabler` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, EventId, EventStorage, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    ///     TaskletStorage};
    /// #
    /// # fn task(_: EventId, _: &mut (), _: &dyn RuntimeApi) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<EventId, (), 0> = TaskletStorage::new();
    /// #
    /// enum Events {
    ///     MyEvent,
    /// }
    ///
    /// impl From<Events> for EventId {
    ///     fn from(value: Events) -> Self {
    ///         match value {
    ///             Events::MyEvent => 1,
    ///         }
    ///     }
    /// }
    ///
    /// static MY_EVENT_STORAGE: EventStorage = EventStorage::new();
    ///
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     # let task_config = TaskletConfig::default();
    ///     # aerugo
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     # aerugo
    ///     #   .create_event(Events::MyEvent.into(), &MY_EVENT_STORAGE)
    ///     #   .expect("Unable to create MyEvent");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///     let task_events = [Events::MyEvent.into()];
    ///
    ///     aerugo
    ///         .subscribe_tasklet_to_events(&task_handle, task_events)
    ///         .expect("Failed to subscribe Task to events");
    /// }
    /// ```
    fn subscribe_tasklet_to_events<C, const COND_COUNT: usize, const EVENT_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<EventId, C, COND_COUNT>,
        events: [EventId; EVENT_COUNT],
    ) -> Result<(), InitError> {
        let tasklet = tasklet_handle.tasklet();

        let event_set = unsafe { EVENT_MANAGER.create_event_set(tasklet.ptr())? };

        for event_id in events {
            let event = match EVENT_MANAGER.get_event(event_id) {
                Some(event) => event,
                None => return Err(InitError::EventNotFound(event_id)),
            };

            // SAFETY: This is safe as long as this function is called only during system initialization.
            unsafe { event.add_set(event_set)? };
        }

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe { tasklet.subscribe(event_set)? };

        Ok(())
    }

    /// Subscribes tasklet to the boolean condition.
    ///
    /// Tasklet subscribes for a state changes in this condition. Changing the value of the
    /// condition will wake up all subscribed tasklets and make them ready to be executed.
    ///
    /// Each tasklet can be subscribed to at maximum one data provider. Condition can have multiple
    /// tasklet registered.
    ///
    /// Strong typing is enforced, tasklet can only be subscribed to events if it processes bool type.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Handle to the target condition.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, BooleanConditionStorage, RuntimeApi, SystemHardwareConfig, TaskletConfig,
    ///     TaskletStorage};
    /// #
    /// # fn task(_: bool, _: &mut (), _: &dyn RuntimeApi) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<bool, (), 0> = TaskletStorage::new();
    /// # static CONDITION_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
    /// #
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     # let task_config = TaskletConfig::default();
    ///     # aerugo
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     # aerugo
    ///     #   .create_boolean_condition(&CONDITION_STORAGE, true)
    ///     #   .expect("Unable to create BooleanCondition");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///     let condition_handle = CONDITION_STORAGE.create_handle().expect("Failed to create Condition handle");
    ///
    ///     aerugo
    ///         .subscribe_tasklet_to_condition(&task_handle, &condition_handle)
    ///         .expect("Failed to subscribe Task to Condition");
    /// }
    /// ```
    fn subscribe_tasklet_to_condition<C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<bool, C, COND_COUNT>,
        condition_handle: &BooleanConditionHandle,
    ) -> Result<(), InitError> {
        let tasklet = tasklet_handle.tasklet();
        let condition = condition_handle.condition();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            tasklet.subscribe(condition)?;
            condition.register_tasklet(tasklet.ptr())?;
        }

        Ok(())
    }

    /// Subscribes tasklet to the cyclic execution.
    ///
    /// Tasklet subscribes for cyclic execution. Tasklet will be executed in specified period,
    /// or will be always ready for execution if such period was not specified. On execution
    /// tasklet won't receive any data, so cycling tasklets are useful mostly as ex. producers or
    /// some periodic housekeeping operations.
    ///
    /// Each tasklet can be subscribed to at maximum on data provider.
    ///
    /// # Generic Parameters
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of tasklet conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `period` - Time period of the execution.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, InitApi, RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage};
    /// #
    /// # fn task(_: (), _: &mut (), _: &dyn RuntimeApi) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<(), (), 0> = TaskletStorage::new();
    /// #
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     # let task_config = TaskletConfig::default();
    ///     # aerugo
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///
    ///     aerugo
    ///         .subscribe_tasklet_to_cyclic(&task_handle, None)
    ///         .expect("Failed to subscribe Task to cyclic execution");
    /// }
    /// ```
    fn subscribe_tasklet_to_cyclic<C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<(), C, COND_COUNT>,
        period: Option<crate::time::MillisDurationU32>,
    ) -> Result<(), InitError> {
        let tasklet = tasklet_handle.tasklet();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            let cyclic_execution =
                CYCLIC_EXECUTION_MANAGER.create_cyclic_execution(tasklet.ptr(), period)?;
            tasklet.subscribe(cyclic_execution)?;
        }

        Ok(())
    }

    /// Sets tasklet condition set.
    ///
    /// Tasklet can use a set of BooleanConditions as a execution condition. Before tasklet is
    /// scheduled and then executed it's condition is checked to verify whether this tasklet is
    /// active. Tasklet will be woken when value of any of the condition in the set changes.
    ///
    /// # Generic Parameters
    /// * `T` - Type of the data.
    /// * `C` - Type of the structure with tasklet context data.
    /// * `COND_COUNT` - Number of conditions.
    ///
    /// # Parameters
    /// * `tasklet` - Handle to the target tasklet.
    /// * `condition` - Set of conditions.
    ///
    /// # Return
    /// `()` if successful, `InitError` otherwise.
    ///
    ///
    /// # Safety
    /// This function shouldn't be called after the system was started, because subscription is safe
    /// only before that.
    ///
    /// # Example
    /// ```
    /// # use aerugo::{Aerugo, BooleanConditionSet, BooleanConditionSetType, BooleanConditionStorage, InitApi,
    ///     RuntimeApi, SystemHardwareConfig, TaskletConfig, TaskletStorage};
    /// #
    /// # fn task(_: (), _: &mut (), _: &dyn RuntimeApi) {}
    /// #
    /// # static TASK_STORAGE: TaskletStorage<(), (), 2> = TaskletStorage::new();
    /// # static CONDITION_X_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
    /// # static CONDITION_Y_STORAGE: BooleanConditionStorage = BooleanConditionStorage::new();
    /// #
    /// fn main() {
    ///     # let (aerugo, _) = Aerugo::initialize(SystemHardwareConfig::default());
    ///     # let task_config = TaskletConfig::default();
    ///     # aerugo
    ///     #   .create_tasklet(TaskletConfig::default(), task, &TASK_STORAGE)
    ///     #   .expect("Unable to create Tasklet");
    ///     # aerugo
    ///     #   .create_boolean_condition(&CONDITION_X_STORAGE, true)
    ///     #   .expect("Unable to create BooleanConditionX");
    ///     # aerugo
    ///     #   .create_boolean_condition(&CONDITION_Y_STORAGE, true)
    ///     #   .expect("Unable to create BooleanConditionY");
    ///     let task_handle = TASK_STORAGE.create_handle().expect("Failed to create Task handle");
    ///     let condition_x_handle = CONDITION_X_STORAGE
    ///         .create_handle()
    ///         .expect("Failed to create ConditionX handle");
    ///     let condition_y_handle = CONDITION_Y_STORAGE
    ///         .create_handle()
    ///         .expect("Failed to create ConditionY handle");
    ///
    ///     let mut condition_set = BooleanConditionSet::<2>::new(BooleanConditionSetType::And);
    ///     condition_set.add(&condition_x_handle);
    ///     condition_set.add(&condition_y_handle);
    ///
    ///     aerugo
    ///         .set_tasklet_conditions(&task_handle, condition_set)
    ///         .expect("Unable to set Task condition set");
    /// }
    fn set_tasklet_conditions<T, C, const COND_COUNT: usize>(
        &'static self,
        tasklet_handle: &TaskletHandle<T, C, COND_COUNT>,
        condition_set: BooleanConditionSet<COND_COUNT>,
    ) -> Result<(), InitError> {
        let tasklet = tasklet_handle.tasklet();

        // SAFETY: This is safe as long as this function is called only during system initialization.
        unsafe {
            condition_set.register_tasklet(tasklet.ptr())?;
            tasklet.set_condition_set(condition_set)?;
        }

        Ok(())
    }

    /// Starts the system.
    ///
    /// This starts an executor that never returns, executing ready tasklets in a loop.
    /// It also enables global interrupts.
    ///
    /// # Safety
    /// This shouldn't be called more than once.
    fn start(&'static self) -> ! {
        // SAFETY: This is safe, because it's called from non-IRQ context, and
        // system time cannot be accessed from IRQ context
        unsafe {
            self.time_source
                .set_system_start()
                .expect("Failed to set system start time");
        }

        self.run()
    }
}

impl RuntimeApi for Aerugo {
    fn emit_event(&'static self, event_id: EventId) -> Result<(), RuntimeError> {
        EVENT_MANAGER.emit(event_id)
    }

    fn cancel_event(&'static self, event_id: EventId) -> Result<(), RuntimeError> {
        EVENT_MANAGER.cancel(event_id)
    }

    fn clear_event_queue(&'static self) {
        EVENT_MANAGER.clear()
    }

    fn get_system_time(&'static self) -> Instant {
        self.time_source.system_time()
    }

    fn set_system_time_offset(&'static self, offset: Duration) -> Result<(), RuntimeError> {
        // SAFETY: This is safe, because it's called from non-IRQ context, and
        // system time cannot be accessed from IRQ context
        unsafe { self.time_source.set_user_offset(offset) }
    }

    /// Returns time elapsed between system initialization and start of the scheduler.
    /// If called before [`Aerugo::start`](crate::Aerugo::start), returns `None`.
    fn get_startup_time(&'static self) -> Option<Duration> {
        self.time_source.startup_duration()
    }

    /// Returns time elapsed since scheduler's start.
    /// If called before [`Aerugo::start`](crate::Aerugo::start), returns `None`.
    fn get_time_since_startup(&'static self) -> Option<Instant> {
        self.time_source.time_since_start()
    }

    fn query_tasks(&'static self) -> core::slice::Iter<TaskletId> {
        todo!()
    }

    fn get_execution_statistics(&'static self, _task_id: TaskletId) -> ExecutionStats {
        todo!()
    }

    fn execute_critical<F, R>(f: F) -> R
    where
        F: FnOnce(CriticalSection) -> R,
    {
        critical_section::with(f)
    }
}
