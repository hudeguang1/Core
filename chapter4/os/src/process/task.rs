use crate::memory::*;
use super::*;
use crate::memory::mapping::{segment::*, Flags};
use crate::trap::*;
pub struct Process {
    pub context_ptr: usize,
    pub task_status: TaskStatus,
    pub memory_set: MemorySet,
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    Exit,
    Running,
    Ready,
}

impl Process {
    pub fn new(elf_data: &[u8], app_id: usize) -> Self {
        let (mut memory_set, entry_point) = MemorySet::from_elf(elf_data);
        let stack = Process::alloc_page_range(&mut memory_set, USER_STACK_SIZE, Flags::READABLE | Flags::WRITABLE);
        let context = Context::new(stack.end.into(), entry_point, true);
        let mut context_ptr: usize = 0;
        unsafe {
            context_ptr = KERNEL_STACK[app_id].push_context(context) as * const _ as usize;
        }
        Process {
            context_ptr,
            task_status: TaskStatus::Ready,
            memory_set,
        }
    }

    pub fn alloc_page_range(memory_set: &mut MemorySet, size: usize, flags: Flags) -> Range<VirtualAddress> {
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x100000 + alloc_size);
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        memory_set.add_segment(
            Segment{
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(true),
            }, 
            None,
        );
        Range::from(range.start..(range.start + size))
    }
}

