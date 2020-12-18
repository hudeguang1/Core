use core::mem::zeroed;
use riscv::register::sstatus::{self, Sstatus, SPP::*};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    /// 通用寄存器
    pub x: [usize; 32],
    /// 保存诸多状态位的特权态寄存器
    pub sstatus: Sstatus,
    /// 保存中断地址的特权态寄存器
    pub sepc: usize,
}

impl Default for Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[allow(unused)]
impl Context {
    pub fn sp(&self) -> usize {
        self.x[2]
    }

    pub fn set_sp(&mut self, value: usize) -> &mut Self {
        self.x[2] = value;
        self
    }

    pub fn ra(&self) -> usize {
        self.x[1]
    }

    pub fn set_ra(&mut self, value: usize) -> &mut Self {
        self.x[1] = value;
        self
    }

    pub fn new(stack_top: usize, entry: usize, is_user: bool) -> Self{
        let mut context = Self::default();
        context.set_sp(stack_top);
        context.sepc = entry;
        context.sstatus = sstatus::read();
        if is_user {
            context.sstatus.set_spp(User);
        } else {
            context.sstatus.set_spp(Supervisor);
        }

        context
    }
}
