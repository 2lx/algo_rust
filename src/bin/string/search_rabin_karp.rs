extern crate algo_rust;

use algo_rust::string::search_rabin_karp::*;

fn main() {
    let rks = RKSearch::new(52, 65713);

    match rks.find_first(b"abcbabcbcbcbabacb", b"bcbcb") {
        Some(n) => println!("{}", n),
        None    => println!("Not found"),
    }
}
