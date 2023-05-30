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
    }

    mod lcd {
        fn lcd_set_text(line_num: i32, str_len: i32, str_ptr: i32) -> i32;
        fn lcd_initialize() -> i32;
        fn lcd_shutdown() -> i32;
        fn lcd_is_initialized() -> i32;
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
