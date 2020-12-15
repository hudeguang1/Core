use super::*;

pub fn write_a() {
    for i in 1..=5 {
        printu!("AAAAA [{}/5]", i);
        sys_yield((i+1) % 2);
    }
    sys_exit(1);
}

pub fn write_b() {
    for i in 1..=3 {
        printu!("BBBBB [{}/3]", i);
        sys_yield((i+1) % 2);
    }
    sys_exit(2);
    
}