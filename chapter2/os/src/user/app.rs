use super::*;

pub fn hello_world() {
    printlnu!("hello world");
    sys_exit(1);
}

pub fn count_sum() {
    let array = [1, 2, 3, 4, 5];
    let mut sum = 0;
    for i in array.iter() {
        sum += i;
    }
    printlnu!("sum = {}", sum);
    sys_exit(2);
}