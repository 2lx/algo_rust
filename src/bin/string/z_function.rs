extern crate algo_rust;

use algo_rust::string::z_function::*;

fn main() {
    let text = b"abacaba";
    let z_values1 = build_z_function_naive(text);
    let z_values2 = build_z_function_effective(text);

    assert_eq!(z_values1, z_values2);

    println!("{}", String::from_utf8_lossy(text));
    for z in &z_values2 { print!("{} ", z); }
    println!("");
}
