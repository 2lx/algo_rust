// https://neerc.ifmo.ru/wiki/index.php?title=Сжатое_суффиксное_дерево
use std::cmp::{min};

use structures::trie_node::TrieNode;
use structures::suffix_trie::{SuffixTrie, SuffixTrieBehaviour};

#[allow(dead_code)]
pub type NodeTypeCompressed = (usize, usize);
#[allow(dead_code)]
pub type TrieTypeCompressed = u8;
#[allow(dead_code)]
pub type TrieNodeCompressed = TrieNode<NodeTypeCompressed>;
#[allow(dead_code)]
pub type SuffixTrieCompressed = SuffixTrie<TrieTypeCompressed, NodeTypeCompressed>;

impl SuffixTrieBehaviour<TrieTypeCompressed, NodeTypeCompressed> for SuffixTrieCompressed
{
    fn new(alphabet: &[u8], data: TrieTypeCompressed) -> SuffixTrieCompressed
    {
        assert!(alphabet.iter().all(|&u| u != data));

        let mut trie = SuffixTrie::default_new(alphabet, data);
        trie.capacity += 1;
        trie
    }

    fn build(&mut self, text: &[u8]) {
        let len = text.len();
        for i in 0..len {
            self.build_suffix(text, i, len)
        }
    }
}

impl SuffixTrieCompressed {
    fn build_suffix(&mut self, text: &[u8], mut new_start: usize, new_finish: usize) {
        let mut cur_index = (text[new_start] - self.min_symbol) as usize;
        let mut cur_node = &mut self.root;

        while cur_node.child_as_ref(cur_index).is_some() {
            let (old_start, old_finish) = cur_node.child_as_ref(cur_index).unwrap().data;
            let eq_len = text[old_start..min(old_finish + 1, new_finish)].iter()
                        .zip(text[new_start..new_finish].iter().chain(b"$"))
                        .take_while(|&(&u1, &u2)| u1 == u2).count();

            if old_start + eq_len <= old_finish { // split at eq_len'th symbol
                let mut new_node = TrieNodeCompressed::new(self.capacity, (old_start, old_start + eq_len - 1));
                let mut old_node = cur_node.take_child(cur_index);
                old_node.data.0 = old_start + eq_len;
                let old_index = (text[old_start + eq_len] - self.min_symbol) as usize;
                new_node.insert_child(old_index, old_node);
                cur_node.insert_child(cur_index, new_node);
            }
            new_start += eq_len;
            cur_node = {cur_node}.child_as_mut(cur_index).unwrap();
            cur_index = if new_start >= new_finish {self.capacity - 1} else {(text[new_start] - self.min_symbol) as usize};
        }

        cur_node.insert_child(cur_index, TrieNodeCompressed::new(self.capacity, (new_start, new_finish)));
    }

    #[allow(dead_code)]
    pub fn contains(&self, _text: &[u8], _pattern: &[u8]) -> bool {
        // let mut start = 0usize;
        // let finish = pattern.len() - 1;
        // let mut cur_index = (pattern[start] - self.min_symbol) as usize;
        // let mut cur_node = &self.root;
        //
        // while cur_node.child_as_ref(cur_index).is_some() {
        //     let (old_start, old_finish) = cur_node.child_as_ref(cur_index).unwrap().data;
        //     let eq_len = text[old_start..min(old_finish + 1, new_finish)].iter()
        //                 .zip(text[new_start..new_finish].iter().chain(b"$"))
        //                 .take_while(|&(&u1, &u2)| u1 == u2).count();
        //
        //     let index = (*c - self.min_symbol) as usize;
        //     if cur_node.child_as_ref(index).is_none() { return false; }
        //     cur_node = {cur_node}.child_as_ref(index).unwrap();
        // }
        true
    }
}

