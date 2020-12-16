use super::*;

const LEN: usize = 100;

pub fn write_a() {
    let p = 3u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else {cur + 1};
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_3 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_3 OK!");
    sys_exit(0);
}

pub fn write_b() {
    let p = 5u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else {cur + 1};
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_5 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_5 OK!");
    sys_exit(1);
    
}

pub fn write_c() {
    let p = 7u64;
    let m = 998244353u64;
    let iter: usize = 200000;
    let mut s = [0u64; LEN];
    let mut cur = 0usize;
    s[cur] = 1;
    for i in 1..=iter {
        let next = if cur + 1 == LEN { 0 } else {cur + 1};
        s[next] = s[cur] * p % m;
        cur = next;
        if i % 10000 == 0 {
            printlnu!("power_7 [{}/{}]", i, iter);
        }
    }
    printlnu!("{}^{} = {}", p, iter, s[cur]);
    printlnu!("Test power_7 OK!");
    sys_exit(2); 
}

pub fn sleep() {
    let current_time = sys_get_time();
    let wait_for = current_time + 10000000;
    while sys_get_time() < wait_for {
        sys_yield(0);
    }
    printlnu!("Test sleep OK!");
    sys_exit(3);
}