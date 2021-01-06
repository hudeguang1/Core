
use crate::scheduler::*;
use core::slice::from_raw_parts_mut;

// pub const STDIN: usize = 0;

pub fn sys_read(fd: usize, buf: *mut u8, len: usize) -> isize {
    let process = SCHEDULER.lock().current();
    if let Some(inode) = process.get_lock().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buf, len) };
        // 尝试读取
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret > 0 {
                return ret;
            }
        }
    }
    -1
}