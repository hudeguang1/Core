mod sys_exit;
mod sys_write;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
use sys_write::*;
use sys_exit::*;

pub fn syscall(id: usize, args: [usize; 3]) -> usize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as usize),
        _ => panic!("id err"),
    }
}