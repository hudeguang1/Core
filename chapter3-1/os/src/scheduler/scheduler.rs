use lazy_static::*;
use crate::trap::Context;
use crate::user::*;
use crate::process::*;
use core::cell::RefCell;
use crate::syscall::*;

pub struct Scheduler {
    pub inner: RefCell<SchedulerInner>,
    
}

pub struct SchedulerInner {
    pub processes: [Process; 3],
    pub current_process: usize,
    pub num_app: usize,
}

unsafe impl Sync for Scheduler {}
lazy_static! {
    pub static ref SCHEDULER: Scheduler = {
        let app_addr = [write_a as usize, write_b as usize, write_c as usize];
        let num_app = app_addr.len();
        let mut processes: [Process; 3] = [
            Process {context_ptr: 0, task_status: TaskStatus::Ready, };
            3
        ];
        
        unsafe {
            for i in 0..app_addr.len() {
                let context = Context::new(USER_STACK[i].get_sp(), app_addr[i] as usize, true);
                processes[i].context_ptr =  KERNEL_STACK[i].push_context(context) as * const _ as usize;
            }
        };
        Scheduler {
            inner: RefCell::new(SchedulerInner {
                processes,
                current_process: 0,
                num_app,
            }),
        }
    };
}

impl Scheduler {
    pub fn set_status(&self, user_app: usize, sys_id: usize){
        if sys_id == SYSCALL_EXIT {
            self.inner.borrow_mut().num_app -= 1;
            self.inner.borrow_mut().processes[user_app].task_status = TaskStatus::Exit;
        } else if sys_id == SYSCALL_YIELD {
            self.inner.borrow_mut().processes[user_app].task_status = TaskStatus::Ready;
        } else {
            self.inner.borrow_mut().processes[user_app].task_status = TaskStatus::Running;
        }
    }

    pub fn get_app_num(&self) -> usize {
        self.inner.borrow_mut().num_app
    }
    
    pub fn get_ptr(&self) -> usize{
        let mut inner = self.inner.borrow_mut();
        let mut next = 0;
        let current = inner.current_process;
        for i in 0..inner.processes.len() {
            if inner.processes[i].task_status == TaskStatus:: Ready {
                next = i;
                break;
            }
        }
        if inner.num_app == 1 {
            return inner.processes[current].context_ptr;
        }
        if next != current {
            inner.processes[current].task_status = TaskStatus::Ready;
            inner.current_process = next;
        }
        inner.processes[next].task_status = TaskStatus::Running;
        inner.processes[next].context_ptr
    }
}