# Pros-rs Robot Simulator

> Simulate VEX V5 robots using the `pros-rs` framework.

## About

Pros-rs is a framework for programming reliable VEX robots. As you create your program, you will likely need to test your code and you may or may not have access to a real robot that's set up like your program expects. The Pros-rs simulator aims to fix this issue by providing a VEX robot simulator for prototyping, testing, and debugging.

## Usage

This simulator uses WebAssembly to run code in a sandbox, independent of system details. C-like functions that are similar to the official PROS API are available from the `pros_v0` WASM import. A full list is available [here](./src/api.rs). These are not meant to be used directly - if you're writing a program that's meant run in the simulator, use a wrapper library that supports using both the official API and the simulator API as a backend.

## Compliling a simulated crate

```sh
RUSTFLAGS="-Clink-arg=--export-table" cargo build --target wasm32-unknown-unknown
```
