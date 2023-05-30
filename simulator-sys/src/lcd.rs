#[link(wasm_import_module = "pros_v0")]
extern "C" {
    pub fn lcd_set_text(line_number: i32, message_length: i32, message_ptr: *const u8);
    pub fn lcd_initialize() -> i32;
    pub fn lcd_shutdown() -> i32;
    pub fn lcd_is_initialized() -> i32;
}
