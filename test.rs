// rustc --target wasm32-unknown-unknown test.rs -o test.wasm

#![no_main]

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::collections::HashMap;

#[link(wasm_import_module = "pros_v0")]
extern "C" {
    fn lcd_print(line_num: usize, str_ptr: *const u8, length: usize);
}

fn allocations() -> &'static mut HashMap<usize, Layout> {
    static mut ALLOCATIONS: Option<HashMap<usize, Layout>> = None;
    unsafe { ALLOCATIONS.get_or_insert_with(HashMap::new) }
}

#[no_mangle]
pub extern "C" fn mem_alloc(length: usize) -> usize {
    let layout = Layout::from_size_align(length, 1).unwrap();
    let ptr = unsafe { alloc(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    allocations().insert(ptr as usize, layout);
    ptr as usize
}

#[no_mangle]
pub extern "C" fn mem_dealloc(ptr: usize) {
    let layout = allocations().remove(&ptr).unwrap();
    unsafe { dealloc(ptr as *mut u8, layout) };
}

#[no_mangle]
pub extern "C" fn initialize() {
    let message = "Hello, world!";
    unsafe {
        lcd_print(0, message.as_ptr(), message.len());
    }
}

#[no_mangle]
pub extern "C" fn print_something(fragment: *const u8, length: usize) {
    let input = unsafe { std::slice::from_raw_parts(fragment, length) };
    let mut message = std::str::from_utf8(input).unwrap().to_string();
    message.push_str("!");
    unsafe {
        lcd_print(0, message.as_ptr(), message.len());
    }
}
