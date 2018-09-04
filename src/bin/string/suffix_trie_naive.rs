// https://neerc.ifmo.ru/wiki/index.php?title=Суффиксный_бор

extern crate algo_rust;
use algo_rust::structures::suffix_trie::{SuffixTrieBehaviour};
use algo_rust::string::suffix_trie_naive::{SuffixTrieNaive, NodeTypeNaive};

fn main() {
    let alphabet = b"az";
    let mut trie = SuffixTrieNaive::new(alphabet, ());

    let text = "xabxabaxbababaax";
    trie.build(text.as_bytes());

    let mut last_lvl = 1;
    let mut collect_result = |_node: &NodeTypeNaive, lvl: usize, index: usize| {
        if lvl > last_lvl {
            print!("{}", (trie.min_symbol + index as u8) as char);
        } else {
            print!("\n{}: {}", lvl.to_string(), (trie.min_symbol + index as u8) as char);
        }
        last_lvl = lvl;
    };
    trie.traverse_preorder(&mut collect_result);
    print!("\n");
}

#[cfg(test)]
fn test(alphabet: &[u8], text: &[u8], expected: &[u8]) {
    let mut trie = SuffixTrieNaive::new(alphabet, ());
    trie.build(text);

    let mut vec: Vec<u8> = Vec::new();
    {
        let mut collect_result = |_node: &NodeTypeNaive, lvl: usize, index: usize|
        {
            vec.extend_from_slice(lvl.to_string().as_bytes());
            vec.push(trie.min_symbol + index as u8);
            vec.push(b'|');
        };
        trie.traverse_preorder(&mut collect_result);
    }
    assert_eq!(vec, expected);
}

#[test]
fn test1() { test(b"az", b"abc", b"1a|2b|3c|1b|2c|1c|"); }

#[test]
fn test2() { test(b"az", b"aaba", b"1a|2a|3b|4a|2b|3a|1b|2a|"); }

#[test]
fn test3() { test(b"az", b"xabxa", b"1a|2b|3x|4a|1b|2x|3a|1x|2a|3b|4x|5a|"); }

#[test]
fn test4() { test(b"az", b"abcbabcbcbcbabacb", b"1a|2b|3a|4c|5b|3c|4b|5a|6b|\
    7c|8b|9c|10b|11c|12b|13a|14b|15a|16c|17b|5c|6b|7c|8b|9a|10b|11a|12c|13b|\
    2c|3b|1b|2a|3b|4a|5c|6b|4c|5b|6c|7b|8c|9b|10a|11b|12a|13c|14b|3c|4b|2c|3b|\
    4a|5b|6a|7c|8b|6c|7b|8c|9b|10c|11b|12a|13b|14a|15c|16b|4c|5b|6a|7b|8a|9c|\
    10b|6c|7b|8a|9b|10a|11c|12b|1c|2b|3a|4b|5a|6c|7b|5c|6b|7c|8b|9c|10b|11a|\
    12b|13a|14c|15b|3c|4b|5a|6b|7a|8c|9b|5c|6b|7a|8b|9a|10c|11b|"); }

#[test]
fn test5() {
    let alphabet = b"az";
    let mut trie = SuffixTrieNaive::new(alphabet, ());

    let text = "abcbabcbcbcbabacbcbacbacbcacbabccbababcbaabcbc";
    trie.build(text.as_bytes());

    assert_eq!(trie.contains(b"bcbab"), true);
    assert_eq!(trie.contains(b"bcbcb"), true);
    assert_eq!(trie.contains(b"bccab"), false);
    assert_eq!(trie.contains(b"aacab"), false);
    assert_eq!(trie.contains(b"babac"), true);
    assert_eq!(trie.contains(b"cccac"), false);
    assert_eq!(trie.contains(b"ccaab"), false);
    assert_eq!(trie.contains(b"bcccb"), false);
    assert_eq!(trie.contains(b"bcacb"), true);
    assert_eq!(trie.contains(b"cbcaa"), false);
    assert_eq!(trie.contains(b"accab"), false);
    assert_eq!(trie.contains(b"bcaac"), false);
    assert_eq!(trie.contains(b"cacab"), false);
    assert_eq!(trie.contains(b"cbcba"), true);
}
