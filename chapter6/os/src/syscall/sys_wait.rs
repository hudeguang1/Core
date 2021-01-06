use crate::scheduler::*;

pub fn sys_wait(pid: isize) -> isize {
    let children = SCHEDULER.lock().current().inner.lock().children.clone();
    if pid == -1 {
        for i in children.iter() {
            if SCHEDULER.lock().find_app(*i) == true {
                return -2;
            }
            return -1;
        }
    }
    0
}