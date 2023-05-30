use crate::*;
use anyhow::Error;
use wasmtime::*;

// (line_num: i32, str_ptr: i32, str_len: i32) -> i32
pub fn lcd_set_text(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    let line_num = args[0].unwrap_i32();
    let str_ptr = args[1].unwrap_i32() as u32;
    let str_len = args[2].unwrap_i32() as u32;

    let errno = caller
        .with_state(|state, caller| {
            let text = state
                .memory()
                .get(
                    caller.as_context_mut(),
                    MemoryLocation::new(str_ptr, str_len),
                )
                .to_string();

            state.lcd_set_text(line_num, &text)?;

            Ok::<_, Errno>(())
        })
        .as_errno();

    errno.update_state(&mut caller);
    ret[0] = errno.is_error().as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_initialize(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    // TODO: does pros set errno on failure here?
    ret[0] = caller
        .with_state(|state, _| state.lcd_initialize())
        .as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_shutdown(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    let errno = caller
        .with_state(|state, _| state.lcd_shutdown())
        .as_errno();

    errno.update_state(&mut caller);
    ret[0] = errno.is_error().as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_is_initialized(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    ret[0] = caller
        .with_state(|state, _| state.lcd.is_enabled())
        .as_wasm();

    Ok(())
}
