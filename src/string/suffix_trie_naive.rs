// https://neerc.ifmo.ru/wiki/index.php?title=Суффиксный_бор

use structures::trie_node::TrieNode;
use structures::suffix_trie::{SuffixTrie, SuffixTrieBehaviour};

#[allow(dead_code)]
pub type NodeTypeNaive = ();
#[allow(dead_code)]
pub type TrieTypeNaive = ();
#[allow(dead_code)]
pub type TrieNodeNaive = TrieNode<NodeTypeNaive>;
#[allow(dead_code)]
pub type SuffixTrieNaive = SuffixTrie<TrieTypeNaive, NodeTypeNaive>;

impl SuffixTrieBehaviour<TrieTypeNaive, NodeTypeNaive> for SuffixTrieNaive
{
    fn build(&mut self, text: &[u8]) {
        for i in 0..text.len() {
            self.build_suffix(&text[i..])
        }
    }
}

impl SuffixTrieNaive {
    fn build_suffix(&mut self, suffix: &[u8]) {
        let mut cur_node = &mut self.root;
        for c in suffix {
            let index = (*c - self.min_symbol) as usize;
            if cur_node.child_as_ref(index).is_none() {
                cur_node.insert_child(index, TrieNodeNaive::new(self.capacity, ()));
            }
            cur_node = {cur_node}.child_as_mut(index).unwrap();
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, pattern: &[u8]) -> bool {
        let mut cur_node = &self.root;
        for c in pattern {
            let index = (*c - self.min_symbol) as usize;
            if cur_node.child_as_ref(index).is_none() { return false; }
            cur_node = {cur_node}.child_as_ref(index).unwrap();
        }
        true
    }
}
