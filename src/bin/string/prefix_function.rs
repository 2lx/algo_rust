extern crate algo_rust;

use algo_rust::string::prefix_function::*;

fn main() {
    let text = b"abcabcd";
    let border_lengths1 = build_prefix_function_naive(text);
    let border_lengths2 = build_prefix_function_effective(text);

    assert_eq!(border_lengths1, border_lengths2);
    println!("{}", String::from_utf8_lossy(text));
    for bl in &border_lengths1 { print!("{} ", bl); }
    println!("");
}
