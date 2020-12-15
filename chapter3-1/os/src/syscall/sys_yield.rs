use crate::process::*;
pub fn sys_yield(xstate: usize) -> ! {
    println!("             yeild!");
    //运行下一个用户程序
    next_app(xstate, 124);
}