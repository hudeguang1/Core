mod sys_exit;
mod sys_write;
mod sys_yield;
mod sys_get_time;

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169;

use sys_write::*;
use sys_exit::*;
use sys_yield::*;
use sys_get_time::*;

pub fn syscall(id: usize, args: [usize; 3]) -> usize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as usize),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        _ => panic!("id err"),
    }
}