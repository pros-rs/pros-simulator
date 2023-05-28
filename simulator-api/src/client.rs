use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum Event {
    Display(DisplayEvent),
    Init(InitEvent),
    Log(LogEvent),
    ProgramFinished,
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
pub enum InitEvent {
    Success,
    Failure { error: String },
}

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub enum LogEvent {
    Info(String),
    Warning(String),
    Error(String),
}
