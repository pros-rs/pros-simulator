use bitflags::bitflags;
use wasmtime::*;

pub const LINE_WIDTH: usize = 40;

#[derive(Clone, Default)]
pub struct LcdState {
    pub callbacks: [Option<TypedFunc<(), ()>>; 3],
    pub status: LcdStatus,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum LcdStatus {
    Disabled,
    Enabled(LcdPressedButtons),
}

impl LcdStatus {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }
}

impl Default for LcdStatus {
    fn default() -> Self {
        Self::Disabled
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
    pub struct LcdPressedButtons: i32 {
        const RIGHT = 1 << 0;
        const CENTER = 1 << 1;
        const LEFT = 1 << 2;
    }
}
