use crate::trap::context::Context;

#[derive(Copy, Clone)]
pub struct Process {
    pub context_ptr: usize,
    pub task_status: TaskStatus,
}
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    Exit,
    Running,
    Ready,
}