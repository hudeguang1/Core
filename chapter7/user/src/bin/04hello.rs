#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{
    sys_fork,
    sys_exec,
    sys_getpid,
    sys_wait,
    sys_yield,
};

#[no_mangle]
unsafe fn main() -> i32 {
    // println!("{}", sys_fork());
    if sys_fork() == 0 {
        println!("Children");
        sys_exec("00power_3");  
    } else {
        println!("Parent");
    }

    loop {
        match sys_wait(-1) {
            -2 => { 
                println!("have son process");
                sys_yield();
            }
            -1 => {
                println!("son process all end");
                break;
            }
            _ => {
                println!("no son process");
                break;
            }
        }
    }
    println!("pid:{}",sys_getpid());
    println!("hello world");
    0
}