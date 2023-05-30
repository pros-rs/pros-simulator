use wasmtime::*;

pub const LINE_WIDTH: usize = 40;

pub enum LcdState {
    Disabled,
    Enabled(LcdButtons),
}

impl LcdState {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }
}

impl Default for LcdState {
    fn default() -> Self {
        Self::Disabled
    }
}

#[derive(Default)]
pub struct LcdButtons {
    pub callbacks: [Option<TypedFunc<(), ()>>; 3],
    pub pressed: [bool; 3],
}
