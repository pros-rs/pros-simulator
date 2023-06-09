pub mod api;
pub mod errno;
pub mod memory;
pub mod robot;
pub mod state;

pub trait BoolExt {
    fn as_i32(&self) -> i32;
    fn as_wasm(&self) -> wasmtime::Val {
        self.as_i32().into()
    }
}

impl BoolExt for bool {
    fn as_i32(&self) -> i32 {
        if *self {
            1
        } else {
            0
        }
    }
}

pub use {
    errno::{AsErrno, Errno},
    memory::{MemoryHandle, MemoryLocation, RobotMemory, WasmPtr},
    robot::Robot,
    state::{
        lcd::{LcdPressedButtons, LcdState, LcdStatus},
        AsState, RobotState, StateWrapper,
    },
};
