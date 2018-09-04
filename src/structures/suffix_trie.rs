use std::cmp::{min, max};
use structures::trie_node::TrieNode;
use std::default::Default;

pub struct SuffixTrie<T, N> {
    pub root: TrieNode<N>,
    pub min_symbol: u8,
    pub capacity: usize,
    pub data: T,
}

impl<T, N> SuffixTrie<T, N>
where N: Default,
{
    #[allow(dead_code)]
    pub fn traverse_preorder<F>(&self, func: &mut F)
    where F: FnMut(&N, usize, usize)
    {
        self.root.traverse_preorder(0, 0, func);
    }

    #[allow(dead_code)]
    pub fn traverse_postorder<F>(&self, func: &mut F)
    where F: FnMut(&N, usize, usize)
    {
        self.root.traverse_postorder(0, 0, func);
    }

    #[allow(dead_code)]
    pub fn default_new(alphabet: &[u8], data: T) -> SuffixTrie<T, N>
    where N: Default,
    {
        let (al_min, al_max) = alphabet.iter()
                .fold((255u8, 0u8), |(smin, smax), &u| (min(smin, u), max(smax, u)));
        let cap = (al_max - al_min + 1) as usize;

        SuffixTrie::<T, N> {
                root: TrieNode::<N>::new(cap, N::default()),
                min_symbol: al_min,
                capacity: cap,
                data: data,
        }
    }
}

pub trait SuffixTrieBehaviour<T, N>
{
    fn new(alphabet: &[u8], data: T) -> SuffixTrie<T, N>
    where N: Default,
    {
        SuffixTrie::default_new(alphabet, data)
    }

    fn build(&mut self, text: &[u8]) -> ();
}

