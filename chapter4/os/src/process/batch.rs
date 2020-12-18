use crate::scheduler::*;

pub fn next_app(sys_id: usize) -> ! {
    extern "C" {
        fn __restore(context: usize);
    }
    println!("hello");
    let context_ptr = SCHEDULER.get_ptr();
    SCHEDULER.set_status(sys_id);
    if SCHEDULER.get_app_num() == 0 {
        panic!("app end")
    }
    
    unsafe {
        __restore(context_ptr);
    };

    panic!("batch end");
}