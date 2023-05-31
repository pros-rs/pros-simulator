//! Bindings to the pros-rs simulator APIs.

#[cfg(not(target_arch = "wasm32"))]
compile_error!("pros-simulator-sys may only be used on WebAssembly targets (add `build.target = \"wasm32-unknown-unknown\"` to .cargo/config.toml)");

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub mod alloc;
mod lcd;

pub use lcd::*;

#[link(wasm_import_module = "pros_v0")]
extern "C" {
    pub fn get_errno() -> i32;
    pub fn panic(msg_ptr: *const u8, msg_len: i32);
}

/// Enables panic error messages by registering a custom panic hook.
///
/// By default, Rust uses the `unreachable` instruction to crash the program:
///
/// ```rs,no_run
/// panic!("Uh oh");
/// // -> "wasm trap: wasm `unreachable` instruction executed"
/// ```
///
/// Running this function replaces the default panic hook with one that uses the real error message:
///
/// ```rs,no_run
/// panic!("Uh oh");
/// // -> "panic: Uh oh"
/// ```
pub fn register_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let msg = info.to_string();
        unsafe {
            panic(msg.as_ptr(), msg.len() as _);
        }
        unreachable!();
    }));
}
