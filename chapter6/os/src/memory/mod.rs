pub mod address;
pub mod config;
pub mod frame_track;
pub mod frame_allcator;
pub mod heap;
pub mod mapping;
pub mod range;
/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;

pub use address::*;
pub use config::*;
pub use heap::*;
pub use range::*;
pub use frame_allcator::FRAME_ALLOCATOR;
pub use frame_track::*;
pub use mapping::memory_set::MemorySet;

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}