// Сочетания из n по k

use std::io::{self, Read};

fn cnk(n: usize, k: usize) -> u64 {
    let mut result = 1u64;
    let mut cur = 2usize;

    for i in k + 1..n + 1 {
        result *= i as u64;
        while cur <= n - k && result % (cur as u64) == 0 {
            result /= cur as u64;
            cur += 1;
        }
    }
    result
}

fn main() {
    let mut buffer = String::with_capacity(100);
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut it = buffer.split_whitespace();
    let n = it.next().expect("").parse::<usize>().unwrap();
    let a = it.next().expect("").parse::<usize>().unwrap();
    let b = it.next().expect("").parse::<usize>().unwrap();

    let result;
    if a == 0 && b == 0 { result = 1; }
    else {
        result = cnk(a + n, n) * cnk(b + n, n);
    }
    println!("{}", result);
}
