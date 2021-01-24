#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::string::String;
use user_lib::{
    sys_fork,
    pipe,
    sys_close,
    sys_write,
    sys_read,
};

#[no_mangle]
pub fn main() -> i32 {
    let (read_fd, write_fd) = pipe();
    if sys_fork() == 0 {
        sys_close(write_fd);
        println!("child");
        let mut buffer = [0u8; 64];
        sys_read(read_fd, &mut buffer);
        let mut res = String::new();
        for ch in buffer.iter() {
            res.push(*ch as char);
        }
        println!("{}", res);
    } else {
        println!("parent");
        //sys_close(read_fd);
        sys_write(write_fd, b"hello_world");
    }
    0
}