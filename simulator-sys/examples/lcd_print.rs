use pros_simulator_sys::*;

#[no_mangle]
pub extern "C" fn initialize() {
    let msg = "Hello, world!";
    unsafe {
        // note: errno is not being checked here
        lcd_shutdown(); // this line will emit a warning
        lcd_initialize();
        lcd_set_background_color(0x000000);
        lcd_set_text_color(0xffffff);
        lcd_set_text(0, msg.as_ptr(), msg.len() as i32);
    }
}
