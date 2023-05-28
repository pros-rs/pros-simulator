use std::collections::HashMap;

use crate::*;
use anyhow::Error;
use pros_simulator_api::client::{DisplayEvent, Event};
use wasmtime::*;

// (line_num: i32, str_ptr: i32, str_len: i32) -> ()
pub fn lcd_print(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    _ret: &mut [Val],
) -> Result<(), Error> {
    let line_num = args[0].unwrap_i32();
    let str_ptr = args[1].unwrap_i32() as u32;
    let str_len = args[2].unwrap_i32() as u32;

    let state = caller.state();
    let state = state.borrow();
    let mem = state.memory();

    let line = mem
        .get(
            caller.as_context_mut(),
            MemoryLocation::new(str_ptr, str_len),
        )
        .to_string();

    state.tx_event.send(Event::Display(DisplayEvent::Update {
        lines_delta: HashMap::from([(line_num.try_into()?, line)]),
    }))?;

    Ok(())
}
