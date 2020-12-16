use super::*;
use core::fmt::{self, Write};

struct UserStdout;

impl Write for UserStdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    UserStdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! printu {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::user::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! printlnu {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::user::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}