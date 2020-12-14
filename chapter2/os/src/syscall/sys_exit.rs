use crate::process::*;
pub fn sys_exit(xstate: usize) -> ! {
    println!("App {} is exited", xstate);
    //运行下一个用户程序
    next_app(xstate);
}