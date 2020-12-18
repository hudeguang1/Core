use super::config::*;
use core::mem::size_of;
use crate::trap::Context;

/// 内核栈
#[repr(align(4096))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct KernelStack{
    data: [u8; KERNEL_STACK_SIZE]
}

pub static mut KERNEL_STACK: [KernelStack; 4] = [
    KernelStack { data: [0; KERNEL_STACK_SIZE], };
    4
];

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    pub fn push_context(&mut self, context: Context) -> *mut Context {
        let stack_top = &self.get_sp() + size_of::<Self>();
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}