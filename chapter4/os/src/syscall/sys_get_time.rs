use crate::trap::*;
pub fn sys_get_time() -> usize {
    timer::get_time() as usize
}