use crate::*;
use pros_simulator_macros::define_api;

pub mod lcd;

type State = StateWrapper;
define_api! {
    wasm_import_module = pros_v0;

    mod lcd {
        fn lcd_print(line_num: i32, str_len: i32, str_ptr: i32);
    }
}
