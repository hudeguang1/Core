// use crate::loader::*;
use crate::scheduler::*;
use crate::fs::*;
use xmas_elf::ElfFile;

pub fn sys_exec(buf: *const u8, len: usize) -> isize {
    let slice = unsafe {
        core::slice::from_raw_parts(buf, len)
    };
    let to_str = core::str::from_utf8(slice).unwrap();
    let app = ROOT_INODE.find(to_str).unwrap();
    let data = app.readall().unwrap();
    let elf = ElfFile::new(data.as_slice()).unwrap();
    SCHEDULER.lock().current().exec(&elf);
    0
}