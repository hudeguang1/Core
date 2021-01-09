use crate::scheduler::*;

pub fn sys_getpid() -> isize {
    SCHEDULER.lock().current().get_pid()
}