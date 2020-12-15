use crate::syscall::*;
use super::context::Context;
use riscv::register::{
    scause::{Exception, Scause, Trap},
    stvec,
};

global_asm!(include_str!("./interrupt.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
    
}

#[no_mangle]
pub fn handle(context: &mut Context, scause: Scause, stval: usize) -> &mut Context{
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => user_envcall(context),
        Trap::Exception(Exception::Breakpoint) => break_point(context),
        _ => {
            panic!("Unsupport trap {:?}, stval: {:x}", scause.cause(), stval);
        }
    }
    context
}

fn break_point(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

fn user_envcall(context: &mut Context) {
    context.sepc += 4;
    context.x[10] = syscall(context.x[17], [context.x[10], context.x[11], context.x[12]] );
}