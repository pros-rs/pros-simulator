use crate::errno::Errno;
use crate::state::lcd::LINE_WIDTH;
use crate::*;
use lazy_static::lazy_static;
use pros_simulator_api::client;
use regex::Regex;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::{cell::RefCell, rc::Rc};
use unicode_segmentation::UnicodeSegmentation;
use wasmtime::*;

pub mod lcd;

pub type StateWrapper = Rc<RefCell<RobotState>>;

pub struct RobotState {
    pub memory: Option<RobotMemory>,
    pub indirect_fn_table: Option<Table>,
    pub tx_event: Sender<client::Event>,
    pub lcd: LcdState,
    // TODO: expose this via api
    pub errno: Errno,
}

impl RobotState {
    pub fn new(tx_event: Sender<client::Event>) -> Self {
        Self {
            memory: None,
            indirect_fn_table: None,
            tx_event,
            lcd: LcdState::default(),
            errno: Errno::Success,
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

    pub fn log(&self, message: client::LogEvent) {
        self.tx_event.send(client::Event::Log(message)).unwrap();
    }
    pub fn info(&self, message: impl Into<String>) {
        self.log(client::LogEvent::Info(message.into()));
    }
    pub fn warn(&self, message: impl Into<String>) {
        self.log(client::LogEvent::Warning(message.into()));
    }
    pub fn error(&self, message: impl Into<String>) {
        self.log(client::LogEvent::Error(message.into()));
    }

    pub fn lcd_initialize(&mut self) -> bool {
        if self.lcd.status.is_enabled() {
            self.warn("Cannot initialize LCD when it's already on");
            return false;
        }
        self.lcd.status = LcdStatus::Enabled(LcdPressedButtons::empty());
        self.tx_event
            .send(client::Event::Display(client::DisplayEvent::Init))
            .unwrap();
        true
    }

    pub fn lcd_shutdown(&mut self) -> Result<(), Errno> {
        if !self.lcd.status.is_enabled() {
            self.warn("Cannot shutdown LCD when it's already off");
            return Err(Errno::ENXIO);
        }
        self.lcd.status = LcdStatus::Disabled;
        self.tx_event
            .send(client::Event::Display(client::DisplayEvent::Deinit))
            .unwrap();
        Ok(())
    }

    /// Trim text to fit on a single line of the LCD. This method is
    /// aware of unicode graphemes and will emit a warning if the text
    /// is too long.
    ///
    /// See also: [`state::lcd::LINE_WIDTH`].
    pub fn trim_text(&self, text: &str) -> String {
        // TODO: how does pros handle newlines?
        lazy_static! {
            static ref NEWLINE: Regex = Regex::new("\r?\n").unwrap();
        }

        let first_line = NEWLINE.split(text).next().unwrap().graphemes(true);
        let trimmed = first_line.take(LINE_WIDTH).collect::<String>();

        if trimmed.len() < text.len() {
            self.warn(format!(
                "Trimming text printed to the LCD (expected < {LINE_WIDTH} characters without newlines): {text}",
            ));
        }

        trimmed
    }

    pub fn lcd_set_text(&mut self, line: i32, text: &str) -> Result<(), Errno> {
        if !self.lcd.status.is_enabled() {
            self.error("Cannot print to the LCD when it's off");
            return Err(Errno::ENXIO);
        }
        if !(0..8).contains(&line) {
            self.error(format!("Cannot print to LCD line {line} (must be 0-7)"));
            return Err(Errno::EINVAL);
        }

        let text = self.trim_text(text);
        self.tx_event
            .send(client::Event::Display(client::DisplayEvent::Update {
                lines_delta: HashMap::from([(line.try_into().unwrap(), text)]),
            }))
            .unwrap();
        Ok(())
    }

    pub fn lcd_clear(&mut self) -> Result<(), Errno> {
        if !self.lcd.status.is_enabled() {
            self.error("Cannot clear the LCD when it's disabled");
            return Err(Errno::ENXIO);
        }

        self.tx_event
            .send(client::Event::Display(client::DisplayEvent::Update {
                lines_delta: (0..8)
                    .map(|line| (line, String::new()))
                    .collect::<HashMap<_, _>>(),
            }))
            .unwrap();
        Ok(())
    }

    pub fn lcd_set_background(&mut self, rgba: i32) {
        if !self.lcd.status.is_enabled() {
            self.error("Cannot set the LCD background when it's disabled");
            return;
        }
        self.tx_event
            .send(client::Event::Display(
                client::DisplayEvent::SetBackgroundColor { rgba: rgba as u32 },
            ))
            .unwrap();
    }

    pub fn lcd_set_text_color(&mut self, rgba: i32) {
        if !self.lcd.status.is_enabled() {
            self.error("Cannot set the LCD text color when it's disabled");
            return;
        }
        self.tx_event
            .send(client::Event::Display(client::DisplayEvent::SetTextColor {
                rgba: rgba as u32,
            }))
            .unwrap();
    }
}

pub trait AsState<T> {
    /// Create a new reference to the state - consider using [`AsState::with_state`] if possible.
    fn state(&self) -> Rc<RefCell<T>>;
    /// Run a function with mutable access to the robot state and the store/caller.
    fn with_state<U>(&mut self, f: impl FnOnce(&mut T, &mut Self) -> U) -> U {
        f(&mut self.state().borrow_mut(), self)
    }
}

impl<T> AsState<T> for Store<Rc<RefCell<T>>> {
    fn state(&self) -> Rc<RefCell<T>> {
        self.data().clone()
    }
}

impl<T> AsState<T> for Caller<'_, Rc<RefCell<T>>> {
    fn state(&self) -> Rc<RefCell<T>> {
        self.data().clone()
    }
}
