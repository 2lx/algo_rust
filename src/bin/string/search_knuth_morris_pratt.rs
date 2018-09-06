extern crate algo_rust;

use algo_rust::string::search_knuth_morris_pratt::*;

fn main() {
    let text = b"abcabcabcdasdada";
    let pattern = b"abca";

    let mut search = KMPSearch::new();
    search.build(text, pattern, b'#');
    match search.find_first(text, pattern) {
        Some(n) => println!("{}", n),
        None    => println!("None"),
    }
}
