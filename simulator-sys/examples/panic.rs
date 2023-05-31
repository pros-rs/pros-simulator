use pros_simulator_sys::*;

#[no_mangle]
pub extern "C" fn initialize() {
    register_panic_hook();
    do_panic();
}

fn do_panic() {
    panic!("Uh oh");
}
