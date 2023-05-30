use pros_simulator_sys::{lcd_initialize, lcd_set_text};

#[no_mangle]
pub extern "C" fn initialize() {
    let msg = "Hello, world!";
    unsafe {
        // note: errno is not being checked here
        lcd_initialize();
        lcd_set_text(0, msg.len() as i32, msg.as_ptr());
    }
}
