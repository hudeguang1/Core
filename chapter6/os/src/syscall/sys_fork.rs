use crate::scheduler::*;

pub fn sys_fork() -> isize {
    let current = SCHEDULER.lock().current();
    let new_process = current.fork();
    let new_pid = new_process.pid;
    // let context = new_process.get_context();
    // unsafe {
    //     (*context).x[10] = 0;
    // }
    SCHEDULER.lock().add_process(new_process);
    new_pid
}