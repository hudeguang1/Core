pub fn hello_world() {
    println!("Hello World!");
}

pub fn count_sum(array: [isize;5]) -> isize {
    let mut a: isize = 0;
    for n in array.iter() {
        a += n;
    }
    a
}
