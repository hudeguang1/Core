mod sys_exit;
mod sys_write;
mod sys_yield;
mod sys_get_time;
mod sys_fork;
mod sys_exec;
mod sys_getpid;
mod sys_wait;
mod sys_read;
mod sys_pipe;
mod sys_close;
mod sys_open;

use alloc::{string::String, vec::Vec};

pub const SYSCALL_OPEN: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_PIPE: usize = 59;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_GET_TIME: usize = 169;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;

use sys_write::*;
use sys_exit::*;
use sys_yield::*;
use sys_get_time::*;
use sys_fork::*;
use sys_exec::*;
use sys_getpid::*;
use sys_wait::*;
use sys_read::*;
use sys_pipe::*;
use sys_close::*;
use sys_open::*;

pub fn syscall(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_EXIT => sys_exit(),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_EXEC => sys_exec(args[0] as *const u8, args[1]),
        SYSCALL_GETPID => sys_getpid(),
        SYSCALL_WAITPID => sys_wait(args[0] as isize),
        SYSCALL_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_PIPE => sys_pipe(args[0] as *mut usize),
        SYSCALL_CLOSE => sys_close(args[0]),
        SYSCALL_OPEN => sys_open(args[0] as *const u8, args[1] as usize),
        _ => panic!("syscall id err"),
    }
}

pub fn check_and_clone_cstr(user: *const u8) -> Result<String, String> {
    if user.is_null() {
        Ok(String::new())
    } else {
        let mut buffer = Vec::new();
        for i in 0.. {
            let addr = unsafe { user.add(i) };
            let data = copy_from_user(addr).ok_or(String::from("SysError::EFAULT"))?;
            if data == 0 {
                break;
            }
            buffer.push(data);
        }
        String::from_utf8(buffer).map_err(|_| String::from("SysError::EFAULT"))
    }
}

pub fn copy_from_user<T>(addr: *const T) -> Option<T> {
    unsafe extern "C" fn read_user<T>(dst: *mut T, src: *const T) -> usize {
        dst.copy_from_nonoverlapping(src, 1);
        0
    }
    let mut dst: T = unsafe { core::mem::zeroed() };
    match unsafe { read_user(&mut dst, addr) } {
        0 => Some(dst),
        _ => None,
    }
}