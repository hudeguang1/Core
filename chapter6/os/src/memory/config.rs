use super::address::*;
use lazy_static::*;


/// 页 / 帧大小，必须是 2^n
pub const PAGE_SIZE: usize = 4096;

/// MMIO 设备段内存区域起始地址
pub const DEVICE_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x1000_0000);
/// MMIO 设备段内存区域结束地址
pub const DEVICE_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x1001_0000);

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);
/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize);
}
/// 操作系统动态分配内存所用的堆大小（8M）
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

extern "C" {
    fn kernel_end();
}