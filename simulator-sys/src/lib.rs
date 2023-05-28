//! Bindings to the pros-rs simulator APIs.

#[cfg(not(target_arch = "wasm32"))]
compile_error!("pros-simulator-sys may only be used on WebAssembly targets (add `build.target = \"wasm32-unknown-unknown\"` to .cargo/config.toml)");

mod lcd;

#[doc(hidden)]
#[cfg(feature = "alloc")]
pub mod alloc;

pub use lcd::*;
