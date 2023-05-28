use anyhow::Result;
use pros_simulator::*;
use wasmtime::*;

fn main() -> Result<()> {
    let wat = include_bytes!("../test.wasm");

    let mut robot = Robot::new(wat)?;

    let init = robot
        .instance
        .get_typed_func::<(i32, i32), ()>(&mut robot.store, "print_something")?;

    let msg_fragment = "Hello, world";

    // get access to wasm memory
    let robot_mem = robot.store.data().memory();

    // alloc & write string to wasm memory
    let location = {
        let robot_mem = robot_mem.borrow_mut();
        let mut slice =
            robot_mem.alloc(robot.store.as_context_mut(), msg_fragment.len() as WasmPtr);
        slice.write(msg_fragment.as_bytes(), 0);
        slice.into_raw()
    };

    init.call(robot.store.as_context_mut(), location.as_wasm_tuple())?;

    // dealloc string from wasm memory
    let robot_mem = robot_mem.borrow_mut();
    robot_mem.get_owned(robot.store.as_context_mut(), location);

    Ok(())
}
