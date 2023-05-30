# pros-simulator-sys

> Low-level bindings for writing applications that run in the pros-rs simulator.

## About

WebAssembly has a limited number of types so some of the bindings don't have ideal signatures and may differ from the official PROS API. You may need to convert some of your values to types like `i32` wheing use these functions. The bindings in this crate are guaranteed to exist when compiled to `wasm32-unknown-unknown` and running in the pros-rs simulator.

## Debugging

### I can't compile this crate

Make sure you're actually compiling to WebAssembly. Create a `.cargo/config.toml` file in your project root and paste [these lines](./.cargo/config.toml) into it.

### Registering callbacks (e.g. `lcd_register_btn0_cb`) crashes the simulator

When compiling to WebAssembly, Rust doesn't expose its indirect function table by default. Create a `.cargo/config.toml` file in your project root and paste [these lines](./.cargo/config.toml) into it.

Alternatively, you can compile with the following environment variable set: `RUSTFLAGS="-Clink-arg=--export-table"`.

### How to inspect the bytecode output

`wasm-tools` from Bytecode Alliance is a very useful tool for debugging WebAssembly. You can install it with Cargo:

```sh
cargo install wasm-tools
```

Then you can convert your compiled `.wasm` file to assembly like this:

```sh
wasm-tools demangle -t ./target/wasm32-unknown-unknown/debug/myprogram.wasm > assembly.wat
```

You can usually find the program's imports at the beginning, the code in the middle, and the exports at the end, and all your strings right after the exports.

If you're having a hard time navigating your assembly:

- Installing the WebAssembly extension for VS Code (the one by the WebAssembly Foundation) will give you syntax highlighting for `.wat` files.
- Use the VS Code "Fold All" command when needed

Some things to confirm if something's breaking:

- Is your assembly exporting a WebAssembly memory buffer called `memory`?
- Is your assembly exporting `initialize`, `opcontrol`,  etc?
- Is your assembly exporting a `funcref` `table` called `__indirect_function_table`?
- Is your assembly exporting the functions `mem_alloc` and `mem_dealloc`? These are usually provided by this crate and allow the simulator to allocate memory that your code can access.
- Compare your assembly's imported functions to the [simulator's exported functions](https://github.com/pros-rs/pros-simulator/blob/main/simulator/src/api.rs). If they are different, this might be a bug in this crate. It is expected that a lot of things (pointers, function references, etc) will be represented as `i32`s.
