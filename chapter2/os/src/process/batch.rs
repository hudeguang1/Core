use super::*;
use crate::trap::Context;
use crate::user::*;

pub fn next_app(user_app: usize) -> ! {
    if user_app == 2 {
        panic!("app end!");
    }

    extern "C" {
        fn __restore(context: usize);
    }

    let app_addr = [hello_world as usize, count_sum as usize];
    unsafe {
        let context = Context::new(USER_STACK.get_sp(), app_addr[user_app] as usize, true);
        __restore(KERNEL_STACK.push_context(context) as usize)
    };

    panic!("batch end");
}