use crate::syscall::*;
use super::context::Context;
use super::timer;
use crate::process::*;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    stvec,
};

global_asm!(include_str!("./interrupt.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
        //sie::set_sext();
    }
}

#[no_mangle]
pub fn handle(context: &mut Context, scause: Scause, stval: usize) -> &mut Context{
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => user_envcall(context),
        Trap::Exception(Exception::Breakpoint) => break_point(context),
        Trap::Exception(Exception::LoadFault) => load_fault(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        _ => {
            panic!(
                "Unresolved trap: {:?}\n{:x?}\nstval: {:x}",
                scause.cause(),
                context,
                stval
            );
        }
    }
    context
}

fn break_point(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer(_context: &mut Context) {
    timer::set_next_timeout();
    next_app(0);
}

fn user_envcall(context: &mut Context) {
    context.sepc += 4;
    context.x[10] = syscall(context.x[17], [context.x[10], context.x[11], context.x[12]]) as usize;
}

fn load_fault(_context: &mut Context) {
    next_app(0);
}