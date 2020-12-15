use lazy_static::*;
use crate::trap::Context;
use crate::user::*;
use crate::process::*;

pub struct Scheduler {
    pub processes: [Process; 2],
    pub current_process: usize,
}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = {
        let mut processes: [Process; 2] = [
            Process {context_ptr: 0, task_status: TaskStatus::Ready, };
            2
        ];
        let app_addr = [write_a as usize, write_b as usize];
        unsafe {
            for i in 0..2 {
                let context = Context::new(USER_STACK[i].get_sp(), app_addr[i] as usize, true);
                processes[i].context_ptr =  KERNEL_STACK[i].push_context(context) as * const _ as usize;
            }
        };
        Scheduler {
            processes,
            current_process: 0,
        }
    };
}
