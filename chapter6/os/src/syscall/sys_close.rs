use crate::scheduler::*;

pub fn sys_close(fd: usize) -> isize {
    let process = SCHEDULER.lock().current();
    process.get_lock().descriptors.remove(fd);
    0
}