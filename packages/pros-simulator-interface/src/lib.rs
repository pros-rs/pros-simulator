pub const LCD_HEIGHT: u32 = 8;
pub const LCD_WIDTH: u32 = 50;
pub type LcdLines = [String; LCD_HEIGHT as usize];

#[derive(Debug)]
pub enum SimulatorEvent {
    Warning(String),

    RobotCodeLoading,
    RobotCodeStarting,
    RobotCodeFinished,
    RobotCodeError { message: String, backtrace: String },

    LcdInitialized,
    LcdUpdated(LcdLines),
    LcdColorsUpdated(u32, u32),
    LcdShutdown,
}