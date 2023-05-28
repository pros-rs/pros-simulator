#[cfg(feature = "runtime")]
pub mod api;
#[cfg(feature = "runtime")]
pub mod memory;
#[cfg(feature = "runtime")]
pub mod simulator;
#[cfg(feature = "runtime")]
pub mod state;

pub mod bindings;

#[cfg(feature = "runtime")]
pub use {
    memory::{MemoryHandle, MemoryLocation, RobotMemory, WasmPtr},
    simulator::Robot,
    state::{CallerExt, RobotState, StateWrapper, StoreExt},
};
