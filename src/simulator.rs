use std::{cell::RefCell, rc::Rc};

use crate::*;
use snafu::{Backtrace, Snafu};
use wasmtime::*;

#[derive(Debug, Snafu)]
#[snafu(display("WASM error: {}", source), context(false))]
pub struct RobotError {
    source: wasmtime::Error,
    backtrace: Backtrace,
}

type Result<T, E = RobotError> = std::result::Result<T, E>;

pub struct Robot {
    pub module: Module,
    pub linker: Linker<RobotState>,
    pub instance: Instance,
    pub store: Store<RobotState>,
}

impl Robot {
    pub fn new(wasm: &[u8]) -> Result<Self> {
        let engine = Engine::new(
            Config::new()
                .debug_info(true)
                .wasm_backtrace_details(WasmBacktraceDetails::Enable)
                // TODO: detect delay starvation
                .epoch_interruption(false)
                // same as vex v5 (optimistic guess)
                .max_wasm_stack(512 * 1024),
        )?;
        let module = Module::new(&engine, wasm)?;

        let robot_state = RobotState::default();
        let mut store = Store::new(&engine, robot_state);

        let mut linker = Linker::new(&engine);
        api::link_api(&mut linker, &module)?;

        let instance = linker.instantiate(&mut store, &module)?;

        store.data_mut().memory = Some(Rc::new(RefCell::new(RobotMemory::new(
            &mut store, &instance,
        ))));

        Ok(Self {
            module,
            linker,
            instance,
            store,
        })
    }
}
