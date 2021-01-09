use super::config::*;


/// 内核栈
#[repr(align(4096))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UserStack{ 
    data: [u8; USER_STACK_SIZE],
}

pub static mut USER_STACK: [UserStack; APP_NUM] = [
    UserStack { data: [0; USER_STACK_SIZE], };
    APP_NUM
];

#[allow(unused)]
impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}