use crate::*;
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct RobotState {
    pub memory: Option<Rc<RefCell<RobotMemory>>>,
}

impl RobotState {
    pub fn memory(&self) -> Rc<RefCell<RobotMemory>> {
        self.memory.as_ref().unwrap().clone()
    }
}
