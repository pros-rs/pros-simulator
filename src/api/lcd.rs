use crate::*;
use anyhow::Error;
use wasmtime::*;

// (line_num: i32, str_ptr: i32, str_len: i32) -> ()
pub fn lcd_print(
    mut caller: Caller<'_, RobotState>,
    args: &[Val],
    _ret: &mut [Val],
) -> Result<(), Error> {
    let line_num = args[0].unwrap_i32();
    let str_ptr = args[1].unwrap_i32() as u32;
    let str_len = args[2].unwrap_i32() as u32;

    let mem = caller.data().memory();
    let mem = mem.borrow_mut();

    let line = mem
        .get(
            caller.as_context_mut(),
            MemoryLocation::new(str_ptr, str_len),
        )
        .to_string();
    println!("lcd_print (line {}): {}", line_num, line);
    Ok(())
}
