use crate::memory::{
    address::*,
    config::*,
    mapping::{Flags, MapType, Mapping, Segment},
    range::Range,
    MemoryResult,
};
use xmas_elf::{
    program::{SegmentData, Type},
    ElfFile,
};
use alloc::{vec, vec::Vec};

/// 一个进程所有关于内存空间管理的信息
#[derive(PartialEq)]
pub struct MemorySet {
    /// 维护页表和映射关系
    pub mapping: Mapping,
    /// 每个字段
    pub segments: Vec<Segment>,
}

impl MemorySet {
    /// 创建内核重映射
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        // 在 linker.ld 里面标记的各个字段的起始点，均为 4K 对齐
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        // 建立字段
        let segments = vec![
            // DEVICE 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(DEVICE_START_ADDRESS..DEVICE_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .text 段，r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata 段，r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },
            // .data 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss 段，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // 剩余内存空间，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;

        // 每个字段在页表中进行映射
        for segment in segments.iter() {
            mapping.map(segment, None)?;
        }
        Ok(MemorySet { mapping, segments })
    }

    /// 替换 `satp` 以激活页表
    ///
    /// 如果当前页表就是自身，则不会替换，但仍然会刷新 TLB。
    pub fn activate(&self) {
        self.mapping.activate();
    }

    /// 添加一个 [`Segment`] 的内存映射
    pub fn add_segment(&mut self, segment: Segment, init_data: Option<&[u8]>) -> MemoryResult<()> {
        // 检测 segment 没有重合
        assert!(!self.overlap_with(segment.page_range()));
        // 映射并将新分配的页面保存下来
        self.mapping.map(&segment, init_data)?;
        self.segments.push(segment);
        Ok(())
    }

    /// 移除一个 [`Segment`] 的内存映射
    ///
    /// `segment` 必须已经映射
    pub fn remove_segment(&mut self, segment: &Segment) -> MemoryResult<()> {
        // 找到对应的 segment
        let segment_index = self
            .segments
            .iter()
            .position(|s| s == segment)
            .expect("segment to remove cannot be found");
        self.segments.remove(segment_index);
        // 移除映射
        self.mapping.unmap(segment);
        Ok(())
    }

    /// 检测一段内存区域和已有的是否存在重叠区域
    pub fn overlap_with(&self, range: Range<VirtualPageNumber>) -> bool {
        for seg in self.segments.iter() {
            if range.overlap_with(&seg.page_range()) {
                return true;
            }
        }
        false
    }

    pub fn from_elf(file: &ElfFile, is_user: bool) -> (Self, usize) {
        // 建立带有内核映射的 MemorySet
        let mut memory_set = MemorySet::new_kernel().unwrap();
        //let file = ElfFile::new(elf_data).unwrap();
        // 遍历 elf 文件的所有部分
        for program_header in file.program_iter() {
            if program_header.get_type() != Ok(Type::Load) {
                continue;
            }
            // 从每个字段读取「起始地址」「大小」和「数据」
            let start = VirtualAddress(program_header.virtual_addr() as usize);
            let size = program_header.mem_size() as usize;
            let data: &[u8] =
                if let SegmentData::Undefined(data) = program_header.get_data(&file).unwrap() {
                    data
                } else {
                    panic!("err")
                };

            // 将每一部分作为 Segment 进行映射
            let segment = Segment {
                map_type: MapType::Framed,
                range: Range::from(start..(start + size)),
                flags: Flags::user(is_user)
                    | Flags::readable(program_header.flags().is_read())
                    | Flags::writable(program_header.flags().is_write())
                    | Flags::executable(program_header.flags().is_execute()),
            };

            // 建立映射并复制数据
            memory_set.add_segment(segment, Some(data)).unwrap();
        }
        (memory_set, file.header.pt2.entry_point() as usize)
    }

    pub fn alloc_page_range(&mut self, size: usize, flags: Flags) -> Range<VirtualAddress> {
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        let mut range = Range::<VirtualAddress>::from(0x1000000..0x1000000 + alloc_size);
        while self.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        self.add_segment(
            Segment{
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(true),
            }, 
            None,
        ).unwrap();
        Range::from(range.start..(range.start + size))
    }

    pub fn copy_memory_set(current_ms: &MemorySet) -> MemorySet {
        let mut memory_set = MemorySet::new_kernel().unwrap();
        for area in current_ms.segments.iter() {
            if area.map_type == MapType::Framed {
                let new_segment = Segment {
                    map_type: MapType::Framed,
                    range: area.range.clone(),
                    flags: area.flags,
                };
                memory_set.add_segment(new_segment, None).unwrap();
                for va in area.range.iter() {
                    current_ms.activate();
                    let src_pa = Mapping::lookup(va).unwrap();
                    memory_set.activate();
                    let dst_pa = Mapping::lookup(va).unwrap();
                    *dst_pa.deref_kernel::<u8>() = *src_pa.deref_kernel::<u8>();
                }
            }
        }
        current_ms.activate();
        memory_set
    }
}