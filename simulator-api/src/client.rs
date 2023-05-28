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
    Init,
    Deinit,
    Update { lines_delta: HashMap<u8, String> },
    Clear,
    SetBackgroundColor { rgba: u32 },
    SetTextColor { rgba: u32 },
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum LogEvent {
    Info(String),
    Warning(String),
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
