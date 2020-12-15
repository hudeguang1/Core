use super::*;
use crate::trap::Context;
use crate::user::*;
use crate::scheduler::*;

pub fn next_app(user_app: usize, sys_id: usize) -> ! {
    extern "C" {
        fn __restore(context: usize);
    }
    let context_ptr = SCHEDULER.get_ptr();
    SCHEDULER.set_status(user_app, sys_id);
    if SCHEDULER.get_app_num() == 0 {
        panic!("app end")
    }
    
    unsafe {
        __restore(context_ptr);
    };

    panic!("batch end");
}