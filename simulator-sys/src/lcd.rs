pub type ButtonCallback = extern "C" fn();

#[link(wasm_import_module = "pros_v0")]
extern "C" {
    pub fn lcd_set_text(line_number: i32, message_ptr: *const u8, message_length: i32) -> bool;
    pub fn lcd_initialize() -> bool;
    pub fn lcd_shutdown() -> bool;
    pub fn lcd_is_initialized() -> bool;
    pub fn lcd_clear() -> bool;
    pub fn lcd_clear_line(line_number: i32) -> bool;
    pub fn lcd_read_buttons() -> i32;
    pub fn lcd_register_btn0_cb(callback: ButtonCallback);
    pub fn lcd_register_btn1_cb(callback: ButtonCallback);
    pub fn lcd_register_btn2_cb(callback: ButtonCallback);
    pub fn lcd_set_background_color(rgba: i32);
    pub fn lcd_set_text_color(rgba: i32);
}
