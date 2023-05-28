use pros_simulator_sys::lcd_print;

#[no_mangle]
pub extern "C" fn initialize() {
    let msg = "Hello, world!";
    unsafe {
        lcd_print(0, msg.len() as i32, msg.as_ptr());
    }
}
