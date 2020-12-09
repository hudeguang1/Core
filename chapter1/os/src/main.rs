//! # 全局属性
//! - '#![no_std]'
//!  禁用标准库
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
mod app;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

use crate::app::*;

fn clear_bss() {
    extern "C" {
        fn text_start();
        fn text_end();
        fn rodata_start();
        fn rodata_end();
        fn data_start();
        fn data_end();
        fn bss_start();
        fn bss_end();
    }
    println!("text_start = 0x{:X}, text_end= 0x{:X}", text_start as usize, text_end as usize);
    println!("rodata_start = 0x{:X}, rodata_end= 0x{:X}", rodata_start as usize, rodata_end as usize);
    println!("data_start = 0x{:X}, data_end= 0x{:X}", data_start as usize, data_end as usize);
    println!("bss_start = 0x{:X}, bss_end= 0x{:X}", bss_start as usize, bss_end as usize);
    (bss_start as usize..bss_end as usize).for_each(|addr| {
        unsafe{ (addr as *mut u8).write_volatile(0) }
    });
}
/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle] //告诉编译器对于此函数禁用编译期间的名称重整
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    hello_world();
    let temp = [1,2,3,4,5];
    println!("sum = {}",count_sum(temp));
    panic!("end of rust_main")
}


