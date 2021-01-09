use crate::scheduler::*;
use crate::fs::*;
use alloc::sync::Arc;
use core::slice::from_raw_parts_mut;

pub fn sys_pipe(pipe: *mut usize) -> isize {
    let process = SCHEDULER.lock().current();
    let (pipe_read, pipe_write) = Pipe::make_pipe();
    let readfd = process.get_lock().descriptors.len();
    process.get_lock().descriptors.push(Arc::new(pipe_read));
    let writefd = process.get_lock().descriptors.len();
    process.get_lock().descriptors.push(Arc::new(pipe_write));
    let fd = unsafe { from_raw_parts_mut(pipe, 2) };
    fd[0] = readfd as usize;
    fd[1] = writefd as usize;
    println!("{}/{}", fd[0], fd[1]);
    0
}