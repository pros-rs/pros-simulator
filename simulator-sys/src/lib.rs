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
}
