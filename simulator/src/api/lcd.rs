use crate::{state::lcd::LcdPressedButtons, *};
use anyhow::{Context, Error};
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
    ret[0] = errno.is_success().as_wasm();

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
    ret[0] = errno.is_success().as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_is_initialized(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    ret[0] = caller
        .with_state(|state, _| state.lcd.status.is_enabled())
        .as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_clear(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    let errno = caller.with_state(|state, _| state.lcd_clear()).as_errno();

    errno.update_state(&mut caller);
    ret[0] = errno.is_success().as_wasm();

    Ok(())
}

// (i32) -> i32
pub fn lcd_clear_line(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    let line_num = args[0].unwrap_i32();

    let errno = caller
        .with_state(|state, _| state.lcd_set_text(line_num, ""))
        .as_errno();

    errno.update_state(&mut caller);
    ret[0] = errno.is_success().as_wasm();

    Ok(())
}

// () -> i32
pub fn lcd_read_buttons(
    mut caller: Caller<'_, StateWrapper>,
    _args: &[Val],
    ret: &mut [Val],
) -> Result<(), Error> {
    ret[0] = caller
        .with_state(|state, _| {
            let pressed_buttons = match &state.lcd.status {
                LcdStatus::Disabled => LcdPressedButtons::empty(),
                LcdStatus::Enabled(buttons) => *buttons,
            };

            pressed_buttons.bits()
        })
        .into();

    Ok(())
}

macro_rules! button_cb {
    ($($button_num:expr),*) => {
        $(
            concat_idents::concat_idents!(fn_name = lcd_register_btn, $button_num, _cb {
                // (i32) -> ()
                pub fn fn_name(
                    mut caller: Caller<'_, StateWrapper>,
                    args: &[Val],
                    _ret: &mut [Val],
                ) -> Result<(), Error> {
                    let callback_ptr = args[0].unwrap_i32();
                    caller.with_state(|state, mut caller| {
                        let callback = state
                            .get_indirect_fn(&mut caller, callback_ptr as u32)
                            .context("Callback pointer does not refer to a valid function")?
                            .typed(&mut caller)
                            .context("Invalid callback signature")?;

                        state.lcd.callbacks[$button_num] = Some(callback);

                        Ok::<_, Error>(())
                    })?;

                    Ok(())
                }
            });
        )*
    };
}

button_cb!(0, 1, 2);

// i32 -> ()
pub fn lcd_set_background_color(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    _ret: &mut [Val],
) -> Result<(), Error> {
    let rgba = args[0].unwrap_i32();

    caller.with_state(|state, _| {
        state.lcd_set_background(rgba);
    });

    Ok(())
}

// i32 -> ()
pub fn lcd_set_text_color(
    mut caller: Caller<'_, StateWrapper>,
    args: &[Val],
    _ret: &mut [Val],
) -> Result<(), Error> {
    let rgba = args[0].unwrap_i32();

    caller.with_state(|state, _| {
        state.lcd_set_text_color(rgba);
    });

    Ok(())
}
