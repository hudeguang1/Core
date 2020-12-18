use lazy_static::*;
use crate::trap::Context;
use crate::syscall::*;
use crate::process::*;
use core::cell::RefCell;
use alloc::vec::*;
use crate::loader::*;
use lazy_static::*;

pub struct Scheduler {
    pub inner: RefCell<SchedulerInner>,
    
}

pub struct SchedulerInner {
    pub processes: Vec<Process>,
    pub current_process: usize,
    pub num_app: usize,
    pub preorder_process: usize,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = {
        let num_app = get_num_app();
        let mut processes: Vec<Process> = Vec::new();
        for i in 0..num_app {
            processes.push(Process::new(
                get_app_data(i),
                i,
            ));
        }
        
        Scheduler {
            inner: RefCell::new(SchedulerInner {
                processes,
                current_process: 0,
                num_app,
                preorder_process: 0,
            }),
        }
    };
}

impl Scheduler {
    pub fn set_status(&self, sys_id: usize){
        let mut inner = self.inner.borrow_mut();
        let user_app = inner.preorder_process;
        if sys_id == SYSCALL_EXIT {
            inner.num_app -= 1;
            inner.processes[user_app].task_status = TaskStatus::Exit;
        }
    }

    pub fn get_app_num(&self) -> usize {
        self.inner.borrow_mut().num_app
    }
    
    pub fn get_ptr(&self) -> usize{
        let mut inner = self.inner.borrow_mut();
        let mut next = 0;
        let app_num = get_num_app();
        let current = inner.current_process;
        println!("{}", current);
        for i in 1..=inner.processes.len() {
            let j = (i + current) % app_num;
            if inner.processes[j].task_status == TaskStatus::Ready {
                next = j;
                println!("{}", next);
                break;
            }
        }
        if inner.num_app == 1 {
            return inner.processes[current].context_ptr;
        }
        if next != current {
                inner.processes[current].task_status = TaskStatus::Ready;
                inner.current_process = next;
                inner.preorder_process = current;
        }
        inner.processes[next].task_status = TaskStatus::Running;
        inner.processes[next].memory_set.activate();
        inner.processes[next].context_ptr
    }
}