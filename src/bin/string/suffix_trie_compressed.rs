extern crate algo_rust;

use algo_rust::structures::suffix_trie::{SuffixTrieBehaviour};
use algo_rust::string::suffix_trie_compressed::{SuffixTrieCompressed, NodeTypeCompressed};

fn main() {
    let alphabet = b"az";
    let end_symbol = b'$';
    let mut sts = SuffixTrieCompressed::new(alphabet, end_symbol);

    let text = b"xabxa";
    sts.build(text);

    let mut collect_result = |node: &NodeTypeCompressed, lvl: usize, _index: usize| {
        let trail_symbol = if node.1 == text.len() {end_symbol} else {text[node.1]};
        println!("lvl:{:>4}: '{}{}'", lvl,
                 String::from_utf8_lossy(&text[node.0..node.1]),
                 trail_symbol as char);
    };
    sts.traverse_preorder(&mut collect_result);
}

#[cfg(test)]
fn test(alphabet: &[u8], end_symbol: u8, text: &[u8], expected: &[u8]) {
    let mut sts = SuffixTrieCompressed::new(alphabet, end_symbol);
    sts.build(text);

    let mut vec: Vec<u8> = Vec::new();
    {
        let mut collect_result = |node: &NodeTypeCompressed, lvl: usize, _index: usize|
        {
            vec.extend_from_slice(lvl.to_string().as_bytes());
            vec.extend_from_slice(&text[node.0..node.1]);
            if node.1 == text.len() { vec.push(end_symbol); }
            else { vec.push(text[node.1]); }
            vec.push(b'|');
        };
        sts.traverse_preorder(&mut collect_result);
    }
    assert_eq!(vec, expected);
}

#[test]
fn test1() { test(b"az", b'$', b"abc", b"1abc$|1bc$|1c$|"); }

#[test]
fn test2() { test(b"az", b'$', b"aaba", b"1a|2aba$|2ba$|2$|1ba$|"); }

#[test]
fn test3() { test(b"az", b'$', b"xabxa", b"1a|2bxa$|2$|1bxa$|1xa|2bxa$|2$|"); }

#[test]
fn test4() { test(b"az", b'$', b"abcbabcbcbcbabacb", b"1a|2b|3acb$|3cb|4abcbcbcbabacb$|\
    4cbcbabacb$|2cb$|1b|2a|3b|4acb$|4cbcbcbabacb$|3cb$|2cb|3ab|4acb$|4cbcbcbabacb$|3cb\
    |4abacb$|4cbabacb$|2$|1cb|2ab|3acb$|3cbcbcbabacb$|2cb|3abacb$|3cbabacb$|2$|"); }
