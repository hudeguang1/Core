#![allow(unused)]
pub fn syscall(id: usize, args: [usize; 3]) -> isize{
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (args[0]), "{x11}" (args[1]), "{x12}" (args[2]),"{x17}" (id)
            : "memory"
            : "volatile"
        );
    }
    ret
}

pub const STDIN: usize = 0;
pub const STDOUT: usize = 1;

pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [ fd, buf.as_ptr() as usize, buf.len() ])
}

pub fn sys_read(fd: usize, buf: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [ fd, buf.as_mut_ptr() as usize, buf.len() ])
}

pub fn sys_exit(xstate: isize) -> isize {
    syscall(SYSCALL_EXIT, [ xstate as usize, 0, 0 ])
}

pub fn sys_yield() -> isize {
    println!("user sys_yeild");
    syscall(SYSCALL_YIELD,[0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0, 0, 0])
}

pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, path.len(), 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0, 0, 0])
}

pub fn sys_wait(pid: isize) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, 0, 0])
}