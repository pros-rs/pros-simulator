//! Implementation details of the simulator's memory.

use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::collections::HashMap;

/// A map of all memory locations that the simulator has allocated.
pub fn allocations() -> &'static mut HashMap<usize, Layout> {
    static mut ALLOCATIONS: Option<HashMap<usize, Layout>> = None;
    unsafe { ALLOCATIONS.get_or_insert_with(HashMap::new) }
}

/// Used by the simulator to allocate memory that can be accessed by the simulated robot.
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

/// Used by the simulator to deallocate memory that it has previoiusly allocated via [`mem_alloc`].
#[no_mangle]
pub extern "C" fn mem_dealloc(ptr: usize) {
    let layout = allocations().remove(&ptr).unwrap();
    unsafe { dealloc(ptr as *mut u8, layout) };
}
