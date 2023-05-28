use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};

use crate::*;
use pros_simulator_api::client;
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
    pub linker: Linker<StateWrapper>,
    pub instance: Instance,
    pub store: Store<StateWrapper>,
}

impl Robot {
    pub fn new(wasm: &[u8], tx_event: Sender<client::Event>) -> Result<Self> {
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

        let robot_state = Rc::new(RefCell::new(RobotState::new(tx_event)));
        let mut store = Store::new(&engine, robot_state);

        let mut linker = Linker::new(&engine);
        api::link_api(&mut linker, &module)?;

        let instance = linker.instantiate(&mut store, &module)?;

        let state = store.state();
        let mut state = state.borrow_mut();
        state.memory = Some(RobotMemory::new(&mut store, &instance));
        state.indirect_fn_table = instance.get_table(&mut store, "__indirect_function_table");

        Ok(Self {
            module,
            linker,
            instance,
            store,
        })
    }
}
