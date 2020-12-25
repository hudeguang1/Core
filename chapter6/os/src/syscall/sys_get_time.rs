use crate::trap::*;
pub fn sys_get_time() -> isize {
    timer::get_time() as isize
}