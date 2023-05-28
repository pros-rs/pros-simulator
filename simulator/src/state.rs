use crate::*;
use pros_simulator_api::client;
use std::sync::mpsc::Sender;
use std::{cell::RefCell, rc::Rc};
use wasmtime::*;

pub type StateWrapper = Rc<RefCell<RobotState>>;

pub struct RobotState {
    pub memory: Option<RobotMemory>,
    pub indirect_fn_table: Option<Table>,
    pub tx_event: Sender<client::Event>,
}

impl RobotState {
    pub fn new(tx_event: Sender<client::Event>) -> Self {
        Self {
            memory: None,
            indirect_fn_table: None,
            tx_event,
        }
    }
    pub fn memory(&self) -> &RobotMemory {
        self.memory.as_ref().unwrap()
    }
    pub fn memory_mut(&mut self) -> &mut RobotMemory {
        self.memory.as_mut().unwrap()
    }
    pub fn get_indirect_fn(&self, store: impl AsContextMut, index: u32) -> Option<Func> {
        const NO_TABLE_EXPORTED: &str = "WASM code did not export an indirect function table (add `build.rustflags = [\"-Clink-arg=--export-table\"]` to Cargo.toml)";
        self.indirect_fn_table
            .expect(NO_TABLE_EXPORTED)
            .get(store, index)
            .and_then(|func| match func {
                Val::FuncRef(func) => func,
                _ => panic!("Expected function reference in indirect function table"),
            })
    }
}

pub trait StoreExt<T> {
    fn state(&self) -> Rc<RefCell<T>>;
}

impl<T> StoreExt<T> for Store<Rc<RefCell<T>>> {
    fn state(&self) -> Rc<RefCell<T>> {
        self.data().clone()
    }
}

pub trait CallerExt<T> {
    fn state(&self) -> Rc<RefCell<T>>;
}

impl<T> CallerExt<T> for Caller<'_, Rc<RefCell<T>>> {
    fn state(&self) -> Rc<RefCell<T>> {
        self.data().clone()
    }
}
