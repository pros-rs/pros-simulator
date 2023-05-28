use anyhow::Result;
use jsonl::Connection;
use pros_simulator::*;
use std::panic;
use std::process;
use std::thread::spawn;
use wasmtime::*;

fn main() -> Result<()> {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let connection = Connection::new_from_stdio();

    spawn(|| {
        run_simulator().unwrap();
    })
    .join()
    .unwrap();

    Ok(())
}

fn run_simulator() -> Result<()> {
    let wat = include_bytes!("../test.wasm");

    let mut robot = Robot::new(wat)?;

    let state = robot.store.state();
    let mut state = state.borrow_mut();

    let init = robot
        .instance
        .get_typed_func::<(), i32>(&mut robot.store, "get_callback")?;

    println!("Getting cb");

    let cb = {
        let cb_ptr = init.call(&mut robot.store.as_context_mut(), ())?;
        state
            .get_indirect_fn(&mut robot.store, cb_ptr as u32)
            .unwrap()
    };

    let msg_fragment = "Hello, world";

    // get access to wasm memory
    let robot_mem = state.memory_mut();

    // alloc & write string to wasm memory
    let location = {
        let mut slice =
            robot_mem.alloc(robot.store.as_context_mut(), msg_fragment.len() as WasmPtr);
        slice.write(msg_fragment.as_bytes(), 0);
        slice.into_raw()
    };

    drop(state);

    println!("Calling");

    cb.typed::<(i32, i32), ()>(robot.store.as_context())
        .unwrap()
        .call(robot.store.as_context_mut(), location.as_wasm_tuple())?;

    // dealloc string from wasm memory

    let state = robot.store.state();
    let mut state = state.borrow_mut();
    let robot_mem = state.memory_mut();
    robot_mem.get_owned(robot.store.as_context_mut(), location);

    Ok(())
}
