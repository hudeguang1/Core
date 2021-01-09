use super::*;
use bitflags::*;
use crate::fs::*;
use crate::scheduler::*;

pub fn sys_open(path: *const u8, flags: usize) -> isize {
    let path = check_and_clone_cstr(path).unwrap();
    let path = path.trim();
    OpenFlags::from_bits_truncate(flags);
    let inode = ROOT_INODE.find(&path).unwrap();
    let process = SCHEDULER.lock().current();
    let fd = process.get_lock().descriptors.len();
    process.get_lock().descriptors.push(inode);
    fd as isize
}

bitflags! {
    struct OpenFlags: usize {
        /// read only
        const RDONLY = 0;
        /// write only
        const WRONLY = 1;
        /// read write
        const RDWR = 2;
        /// create file if it does not exist
        const CREATE = 1 << 6;
        /// error if CREATE and the file exists
        const EXCLUSIVE = 1 << 7;
        /// truncate file upon open
        const TRUNCATE = 1 << 9;
        /// append on each write
        const APPEND = 1 << 10;
        /// close on exec
        const CLOEXEC = 1 << 19;
    }
}