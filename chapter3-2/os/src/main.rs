#![no_std]
//! 
//! -'#[no_main]'
//! 不使用'main'函数等全部Rust-level入口点来作为程序入口
#![no_main]
//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//! - `#![feature(llvm_asm)]`  
//!   内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]


#[macro_use]
mod console;
mod panic;
mod sbi;
mod trap;
mod process;
mod syscall;
mod user;
mod scheduler;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));


fn clear_bss() {
    extern "C" {
        fn bss_start();
        fn bss_end();
    }
    (bss_start as usize..bss_end as usize).for_each(|addr| {
        unsafe{ (addr as *mut u8).write_volatile(0) }
    });
}


/// Rust 的入口函数
#[no_mangle] //告诉编译器对于此函数禁用编译期间的名称重整
pub extern "C" fn rust_main() {
    //clear_bss();
    trap::init();
    process::next_app(0);
}


