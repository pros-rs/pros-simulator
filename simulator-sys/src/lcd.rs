#[link(wasm_import_module = "pros_v0")]
extern "C" {
    pub fn lcd_print(line_number: i32, message_length: i32, message_ptr: *const u8);
}
