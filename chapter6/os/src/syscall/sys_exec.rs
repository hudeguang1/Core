use crate::loader::*;
use crate::scheduler::*;

pub fn sys_exec(buf: *const u8, len: usize) -> isize {
    let slice = unsafe {
        core::slice::from_raw_parts(buf, len)
    };
    let to_str = core::str::from_utf8(slice).unwrap();
    //println!("{:#?}", to_str);
    let elf_data = get_app_data_by_name(to_str).unwrap();
    SCHEDULER.lock().current().exec(elf_data);
    0
}