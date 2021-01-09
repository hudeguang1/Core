use crate::process::*;
use super::*;
pub fn sys_yield() -> ! {
    //运行下一个用户程序
    next_app(SYSCALL_YIELD);
}