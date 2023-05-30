use std::{cell::RefCell, rc::Rc, sync::mpsc::Sender};

use crate::*;
use pros_simulator_api::client::{Event, ProgramType};
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
    pub fn new(wasm: &[u8], tx_event: Sender<Event>) -> Result<Self> {
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

        let robot_state = Rc::new(RefCell::new(RobotState::new(tx_event.clone())));
        let mut store = Store::new(&engine, robot_state);

        let mut linker = Linker::new(&engine);
        api::link_api(&mut linker, &module)?;

        let instance = linker.instantiate(&mut store, &module)?;

        store.with_state(|state, store| {
            state.memory = Some(RobotMemory::new(store, &instance));
            state.indirect_fn_table = instance.get_table(store, "__indirect_function_table");
        });

        tx_event.send(Event::SimulatorRunning).unwrap();

        Ok(Self {
            module,
            linker,
            instance,
            store,
        })
    }

    fn broadcast_program_finish(&mut self, program_type: ProgramType, error: Option<&RobotError>) {
        self.store.with_state(|state, _| {
            state
                .tx_event
                .send(Event::ProgramFinish {
                    program_type,
                    error: error.map(|err| err.to_string()),
                })
                .unwrap();
        });
    }

    fn init_inner(&mut self) -> Result<()> {
        let Some(init_fn) = self
            .instance
            .get_func(&mut self.store, "initialize") else {
            return Ok(());
        };

        let init_fn = init_fn.typed::<(), ()>(&mut self.store)?;
        init_fn.call(&mut self.store, ())?;

        Ok(())
    }

    pub fn initialize(&mut self) -> Result<()> {
        let res = self.init_inner();
        self.broadcast_program_finish(ProgramType::Initialize, res.as_ref().err());
        res
    }
}
