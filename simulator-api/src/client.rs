use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    /// LCD-related events
    Display(DisplayEvent),
    Log(LogEvent),
    /// A user-provided function (e.g. initialize, autonomous, opcontrol) has finished.
    ProgramFinish {
        program_type: ProgramType,
        error: Option<String>,
    },
    /// The simulator has finished setup and is now running user code.
    SimulatorRunning,
    /// The simulator has finished and will no longer run user code.
    SimulatorStopped,
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum DisplayEvent {
    /// lcd_initialize
    Init,
    /// lcd_shutdown
    Deinit,
    /// Some lines on the LCD have changed.
    Update { lines_delta: HashMap<u8, String> },
    /// lcd_clear
    Clear,
    /// lcd_set_background_color
    SetBackgroundColor { rgba: u32 },
    /// lcd_set_text_color
    SetTextColor { rgba: u32 },
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum LogEvent {
    /// Information is available
    Info(String),
    /// Something isn't right, but it's not enough to crash
    Warning(String),
    /// An error has occurred
    Error(String),
}

#[derive(Serialize, Deserialize)]
pub enum ProgramType {
    Initialize,
    Autonomous,
    OpControl,
    CompetitionInitialize,
    Disabled,
}
