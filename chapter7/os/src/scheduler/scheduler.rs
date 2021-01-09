use super::*;
use lazy_static::*;
use crate::syscall::*;
use crate::process::*;
use alloc::sync::Arc;
use alloc::collections::LinkedList;


pub struct Scheduler {
    pub processes: LinkedList<Arc<Process>>,
    pub current_process: Option<Arc<Process>>,
    pub num_app: LinkedList<isize>,
    //pub preorder_process: Option<Arc<Process>>,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    /// 全局的 [`scheduler`]
    pub static ref SCHEDULER: Lock<Scheduler> = Lock::new(Scheduler::new());
}

#[allow(unused)]
impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: LinkedList::new(),
            current_process: None,
            num_app: LinkedList::new(),
            //preorder_process: None,
        }
    }
    pub fn find_app(&self, id: isize) -> bool {
        self.num_app.contains(&id)
    }
    
    pub fn add_process(&mut self, process: Arc<Process>) {
        self.processes.push_back(process);
    }

    pub fn get_next(&mut self) -> Option<Arc<Process>> {
        if let Some(process) = self.processes.pop_front() {
            Some(process)
        } else {
            None
        }
    }
  
    pub fn remove_process(&mut self, process: &Arc<Process>) {
        let mut removed = self.processes.drain_filter(|p| p == process);
        assert!(removed.next().is_some() && removed.next().is_none())
    }

    pub fn current(&self) -> Arc<Process> {
        self.current_process.as_ref().unwrap().clone()
    }

    pub fn get_ptr(&mut self, sys_id: usize) -> usize {
        if let Some(process) = self.get_next() {
            if sys_id != SYSCALL_EXIT && self.current_process.is_none() == false {
                let current_process = self.current();
                self.add_process(current_process);
            }
            let context_ptr = process.prepare();
            self.current_process = Some(process);
            context_ptr
        } else {
            if sys_id == SYSCALL_EXIT {
                panic!("app end");
            } else {
                self.current_process.as_ref().unwrap().inner.lock().context_ptr
            }
        }
    }


    // pub fn set_status(&self, sys_id: usize){
    //     let mut inner = self.inner.borrow_mut();
    //     let user_app;
    //     if inner.num_app == 1 {
    //         user_app = inner.current_process;
    //     } else {
    //         user_app = inner.preorder_process;
    //     }
    //     if sys_id == SYSCALL_EXIT {
    //         inner.num_app -= 1;
    //         println!("\x1b[1;31mapp_num: '{}', exit_NO: {}\x1b[0m", inner.num_app, user_app);
    //         inner.processes[user_app].task_status = TaskStatus::Exited;
    //     }
    // }
    // pub fn get_ptr(&self) -> usize{
    //     let mut inner = self.inner.borrow_mut();
    //     let mut next = 0;
    //     let app_num = get_num_app();
    //     let current = inner.current_process;
    //     for i in 0..inner.processes.len() {
    //         let j = i % app_num;
    //         if inner.processes[j].task_status == TaskStatus::Ready {
    //             next = j;
    //             break;
    //         }
    //     }
    //     if inner.num_app == 1 {
    //         return inner.processes[current].context_ptr;
    //     }
    //     if next != current {
    //             inner.processes[current].task_status = TaskStatus::Ready;
    //             inner.current_process = next;
    //             inner.preorder_process = current;
    //     }
    //     inner.processes[next].task_status = TaskStatus::Running;
    //     inner.processes[next].memory_set.activate();
    //     inner.processes[next].context_ptr
    // }
}