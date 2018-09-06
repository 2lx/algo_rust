extern crate algo_rust;

use algo_rust::number::fibonacci_iterator::*;

fn main() {
    for i in Fibonacci::<usize>::new().take(40) {
        println!("{}", i);
    }
}
