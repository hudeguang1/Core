#![no_std]
//! 
//! -'#[no_main]'
//! 不使用'main'函数等全部Rust-level入口点来作为程序入口
#![no_main]
#![feature(alloc_error_handler)]
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
#![feature(drain_filter)]


#[macro_use]
mod console;
mod panic;
mod sbi;
mod trap;
mod memory;
mod process;
mod syscall;
mod scheduler;
mod drivers;
mod fs;

extern crate alloc;

use crate::process::*;
use alloc::sync::Arc;
use crate::scheduler::*;
use crate::memory::*;
use crate::fs::*;
use xmas_elf::ElfFile;

global_asm!(include_str!("entry.asm"));



/// Rust 的入口函数
#[no_mangle] //告诉编译器对于此函数禁用编译期间的名称重整
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) {
    memory::init();
    trap::init();
    drivers::init(dtb_pa);
    fs::init();
    // let process1 = Arc::new(Process::new(get_app_data_by_name("04hello").unwrap()));
    // let process2 = Arc::new(Process::new(get_app_data_by_name("01power_5").unwrap()));
    // let process3 = Arc::new(Process::new(get_app_data_by_name("02power_7").unwrap()));
    // let process4 = Arc::new(Process::new(get_app_data_by_name("03sleep").unwrap()));
    let process1 = Arc::new(create_user_process("opentest"));

    SCHEDULER.lock().add_process(process1);
    // SCHEDULER.lock().add_process(process2);
    // SCHEDULER.lock().add_process(process3);
    // SCHEDULER.lock().add_process(process4);

    process::next_app(0);

}

/// 创建一个用户进程，从指定的文件名读取 ELF
pub fn create_user_process(name: &str) -> Process {
    // 从文件系统中找到程序
    let app = ROOT_INODE.find(name).unwrap();
    // 读取数据
    let data = app.readall().unwrap();
    // 解析 ELF 文件
    let elf = ElfFile::new(data.as_slice()).unwrap();
    // 利用 ELF 文件创建进程，映射空间并加载数据
    let process = Process::new(&elf, true);
    process
}
