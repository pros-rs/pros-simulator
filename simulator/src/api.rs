use crate::*;
use anyhow::Error;
use pros_simulator_macros::define_api;
use wasmtime::*;

pub mod lcd;

type State = StateWrapper;
define_api! {
    wasm_import_module = pros_v0;

    mod self {
        fn get_errno() -> i32;
        fn panic(msg_ptr: i32, msg_len: i32);
    }

    mod lcd {
        fn lcd_set_text(line_num: i32, str_len: i32, str_ptr: i32) -> i32;
        fn lcd_initialize() -> i32;
        fn lcd_shutdown() -> i32;
        fn lcd_is_initialized() -> i32;
        fn lcd_clear() -> i32;
        fn lcd_clear_line(line_num: i32) -> i32;
        fn lcd_read_buttons() -> i32;
        fn lcd_register_btn0_cb(callback: i32);
        fn lcd_register_btn1_cb(callback: i32);
        fn lcd_register_btn2_cb(callback: i32);
        fn lcd_set_background_color(rgba: i32);
        fn lcd_set_text_color(rgba: i32);
    }
}

// () -> i32
pub fn get_errno(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    ret[0] = caller
        .with_state(|state, _| state.errno)
        .as_errno_i32()
        .into();

    Ok(())
}

// (i32, i32) -> ()
pub fn panic(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    _ret: &mut [Val],
) -> Result<(), Error> {
    let msg_ptr = args[0].unwrap_i32();
    let msg_len = args[1].unwrap_i32();

    let message = caller.with_state(|state, caller| {
        state
            .memory()
            .get(
                caller.as_context_mut(),
                MemoryLocation::new(msg_ptr as _, msg_len as _),
            )
            .to_string()
    });
    Err(anyhow::anyhow!("panic: {message}"))
}
