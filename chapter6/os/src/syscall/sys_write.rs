use crate::scheduler::*;
use core::slice::from_raw_parts_mut;

// const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *mut u8, len: usize) -> isize {
    // match fd {
    //     FD_STDOUT => {
    //         let slice = unsafe { core::slice::from_raw_parts(buf, len) };
    //         let str = core::str::from_utf8(slice).unwrap();
    //         print!("{}", str);
    //         len as isize
    //     },
    //     _ => {
    //         panic!("fd error")
    //     }
    // }
    let process = SCHEDULER.lock().current();
    if let Some(inode) = process.get_lock().descriptors.get(fd) {
        // 从系统调用传入的参数生成缓冲区
        let buffer = unsafe { from_raw_parts_mut(buf, len) };
        // 尝试写入
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return ret;
            }
        }
    }
    -1
}