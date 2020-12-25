use crate::process::next_app;
use super::SYSCALL_YIELD;
use crate::sbi::*;

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd == STDIN {
        let mut c = 0;
        loop {
            c = console_getchar();
            if c == 0 {
                next_app(SYSCALL_YIELD);
            } else {
                break;
            }
        }

        let ptr = buf as *mut u8;

        let ch = c as u8;
        let slice = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        unsafe { slice.as_mut_ptr().write_volatile(ch); }
    }
    1
}