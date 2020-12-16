use crate::process::*;
pub fn sys_yield() -> ! {
    println!("             yeild!");
    //运行下一个用户程序
    next_app(124);
}