use crate::memory::*;
use super::*;
use crate::memory::mapping::*;
use crate::trap::*;
use alloc::{sync::Arc, vec, vec::Vec};
use spin::Mutex;
use core::mem::size_of;
use xmas_elf::ElfFile;
use crate::fs::*;

pub struct Process {
    pub pid: isize,
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    pub stack: Range<VirtualAddress>,
    pub context_ptr: usize,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
    pub is_user: bool,
    pub children: Vec<isize>,
    pub descriptors: Vec<Arc<dyn INode>>
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Exited,
    Running,
    Waiting,
    Ready,
}

static mut PROCESS_COUNTER: isize = 0;

impl PartialEq for Process {
    fn eq(&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for Process {}

impl Process {
    pub fn new(file: &ElfFile, is_user: bool) -> Self {
        let (mut memory_set, entry_point) = MemorySet::from_elf(file, is_user);
        let stack = memory_set.alloc_page_range(USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), entry_point, true);
        let context_ptr;
        unsafe {
            context_ptr = KERNEL_STACK[PROCESS_COUNTER as usize].push_context(context) as * const _ as usize
        }
        Process {
            pid: unsafe {
                PROCESS_COUNTER += 1;
                PROCESS_COUNTER
            },
            inner: Mutex::new(ProcessInner{
                stack,
                context_ptr, 
                task_status: TaskStatus::Ready,
                memory_set,
                is_user: true,
                children: Vec::new(),
                descriptors: vec![STDIN.clone(), STDOUT.clone()],
            }),  
        }
    }

    pub fn get_pid(&self) -> isize {
        self.pid
    }

    pub fn prepare(&self) -> usize {
        self.inner.lock().memory_set.activate();
        self.inner.lock().context_ptr
    }

    pub fn get_context(&self) -> *mut Context {
        unsafe {
            let stack_top = KERNEL_STACK[(self.pid - 1) as usize].data.as_ptr() as usize + size_of::<KernelStack>();
            (stack_top - size_of::<Context>()) as *mut Context
        }       
    }


    pub fn set_status(&mut self, status: TaskStatus) {
        self.inner.lock().task_status = status;
    }

    pub fn fork(self: &Arc<Process>) -> Arc<Process> {
        let memory_set = MemorySet::copy_memory_set(&self.inner.lock().memory_set);
        let task_status = TaskStatus::Ready;
        let is_user = self.inner.lock().is_user;

        let pid =  unsafe {
            PROCESS_COUNTER += 1;
            PROCESS_COUNTER
        };
        let context_ptr;
        unsafe {
            for i in 0..KERNEL_STACK_SIZE {
                KERNEL_STACK[(pid) as usize].data[i] = KERNEL_STACK[(self.pid) as usize].data[i];
            }
            let stack_top = KERNEL_STACK[pid as usize].data.as_ptr() as usize + size_of::<KernelStack>();
            context_ptr = (stack_top - size_of::<Context>()) as usize;
        }
        let stack = self.inner.lock().stack.clone();
        let process = Arc::new(Process {
            pid,
            inner: Mutex::new(ProcessInner {
                stack,
                context_ptr,
                task_status,
                memory_set,
                is_user,
                children: Vec::new(),
                descriptors: self.get_lock().descriptors.clone(),
            }),
        });
        self.inner.lock().children.push(pid);
        process
    }

    pub fn exec(&self, file: &ElfFile) {
        let (mut memory_set, entry_point) = MemorySet::from_elf(file, true);
        let stack = memory_set.alloc_page_range(USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), entry_point, true);
        self.inner.lock().context_ptr = unsafe {
            KERNEL_STACK[self.pid as usize].push_context(context) as * const _ as usize 
        };
        memory_set.activate();
        self.inner.lock().stack = stack;
        self.inner.lock().memory_set = memory_set;
        self.inner.lock().children = Vec::new();
    }

    /// 上锁并获得可变部分的引用
    pub fn get_lock(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }
}

