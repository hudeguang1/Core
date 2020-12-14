#![allow(unused)]
pub fn syscall(id: usize, args: [usize; 3]) -> usize{
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]),"{x17}" (id)
            : "memory"
            : "volatile");
    }
    ret
}

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

pub fn sys_write(fd: usize, buf: &[u8]) -> usize {
    syscall(SYSCALL_WRITE, [ fd, buf.as_ptr() as usize, buf.len() ])
}

pub fn sys_exit(xstate: usize) -> usize {
    syscall(SYSCALL_EXIT, [ xstate, 0, 0 ])
}