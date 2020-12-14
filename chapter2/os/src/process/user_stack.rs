use super::config::*;


/// 内核栈
#[repr(align(4096))]
#[repr(C)]
pub struct UserStack{ 
    data: [u8; USER_STACK_SIZE],
}

pub static mut USER_STACK: UserStack = UserStack{ data: [0; USER_STACK_SIZE] };

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}