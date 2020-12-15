use super::*;
use crate::trap::Context;
use crate::user::*;
use crate::scheduler::*;

pub fn next_app(user_app: usize) -> ! {
    if user_app == 2 {
        panic!("app end!");
    }

    extern "C" {
        fn __restore(context: usize);
    }

    unsafe {
        __restore(SCHEDULER.processes[user_app].context_ptr)
    };

    panic!("batch end");
}