// https://neerc.ifmo.ru/wiki/index.php?title=Алгоритм_Укконена
// http://acm.mipt.ru/twiki/bin/view/Algorithms/UkkonenCPP
use std::cmp::{min, max};

type NodeID = u32;
type TextIndex = u32;
type TriePoint = (NodeID, TextIndex);
const INFINITE: u32 = std::u32::MAX;

pub struct Node {
    pub id: NodeID,
    pub suffix_link: Option<NodeID>,
    pub links: Vec<Option<Link>>,
}

impl Node {
    pub fn new(cap: usize, suffix_link: Option<NodeID>) -> Self {
        Self{ id: 0,
              suffix_link: suffix_link,
              links: (0..cap).map(|_| None).collect::<Vec<Option<Link>>>(),
        }
    }

    pub fn link(&mut self, char_index: usize, link: Link) {
        self.links[char_index] = Some(link);
    }
}

pub struct Link {
    start: TextIndex,
    finish: TextIndex,
    to: NodeID,
}

impl Link {
    pub fn new(start: TextIndex, finish: TextIndex, to: NodeID) -> Self {
        Self{ start: start,
              finish: finish,
              to: to,
        }
    }

    pub fn length(&self) -> TextIndex {
        self.finish - self.start
    }
}

pub struct Arena<T> {
    pub slots: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { slots: Vec::<T>::new(), }
    }

    pub fn push(&mut self, data: T) -> usize {
        self.slots.push(data);
        self.slots.len() - 1
    }

    pub fn slot_as_ref(& self, index: usize) -> & T {
        & self.slots[index]
    }

    pub fn slot_as_mut(&mut self, index: usize) -> &mut T {
        &mut self.slots[index]
    }
}

pub struct SuffixTrie {
    min_symbol: u8,
    capacity: usize,
    nodes: Arena<Node>,
    // dummy_id: NodeID,
    root_id: NodeID,
}

impl SuffixTrie {
    pub fn new(alphabet: &[u8]) -> Self {
        let (al_min, al_max) = alphabet.iter()
                .fold((255u8, 0u8), |(smin, smax), &u| (min(smin, u), max(smax, u)));
        let cap = (al_max - al_min + 1) as usize;

        let mut nodes = Arena::<Node>::new();
        let dummy_id = nodes.push(Node::new(cap, None)) as NodeID;
        let root_id = nodes.push(Node::new(cap, Some(dummy_id))) as NodeID;

        nodes.slot_as_mut(root_id as usize).id = root_id;
        for i in 0..cap {
            nodes.slot_as_mut(dummy_id as usize).link(i, Link::new(0, 1, root_id));
        }

        Self {  min_symbol: al_min,
                capacity: cap,
                nodes: nodes,
                // dummy_id: dummy_id,
                root_id: root_id,
        }
    }

    pub fn node_as_ref(& self, id: NodeID) -> & Node {
        self.nodes.slot_as_ref(id as usize)
    }

    pub fn node_as_mut(&mut self, id: NodeID) -> &mut Node {
        self.nodes.slot_as_mut(id as usize)
    }

    fn link_index(& self, text: &[u8], index: TextIndex) -> usize {
        (text[index as usize] - self.min_symbol) as usize
    }

    fn link_as_ref(& self, text: &[u8], point: TriePoint) -> & Option<Link> {
        let l_index = self.link_index(text, point.1);
        & self.node_as_ref(point.0).links[l_index]
    }

    pub fn build(&mut self, text: &[u8]) {
        let mut active_point: TriePoint = (self.root_id, 0 as TextIndex);

        for i in 0..text.len() {
            active_point = self.update(active_point, i as TextIndex, text);
            // println!("AP1: {}, {}", active_point.0, active_point.1);
            active_point = self.canonize(active_point, (i + 1) as TextIndex, text);
            // println!("AP2: {}, {}", active_point.0, active_point.1);
            // println!("");
        }
    }

    fn canonize(& self, point: TriePoint, finish: TextIndex, text: &[u8]) -> TriePoint {
        if point.1 >= finish { return point }

        let (mut node_id, mut start) = point;
        let lopt = self.link_as_ref(text, (node_id, start));

        if lopt.is_none() {
            // println!("CANON: ERROR!!");
        }

        let mut link = lopt.as_ref().unwrap();
        while finish - start >= link.length() {
            start += link.length();
            node_id = link.to;
            // println!("CANONWH: start={}, node_id={}", start, node_id);
            if start < finish {
                link = self.link_as_ref(text, (node_id, start)).as_ref().unwrap();
            }
        }
        (node_id, start)
    }

    fn test_and_split(&mut self, point: TriePoint, finish: TextIndex, text: &[u8]) -> (bool, NodeID) {
        if point.1 >= finish {
            return (self.link_as_ref(text, (point.0, finish)).is_some(), point.0);
        }

        let link_start;
        let link_finish;
        let link_to;
        match self.link_as_ref(text, point) {
            &Some(ref l) => {
                link_start = l.start;
                link_finish = l.finish;
                link_to = l.to;
            },
            &None        => {
                // println!("TAS: ERROR!!");
                return (false, 0);
            },
        }

        let text_long_index = link_start + finish - point.1;

        if text[finish as usize] == text[text_long_index as usize] {
            return (true, point.0);
        }

        let cap = self.capacity;
        let middle_id: NodeID = self.nodes.push(Node::new(cap, None)) as NodeID;

        let text_index1 = self.link_index(text, link_start);
        self.node_as_mut(point.0).link(text_index1, Link::new(link_start, text_long_index, middle_id));

        let text_index2 = self.link_index(text, text_long_index);
        self.node_as_mut(middle_id).link(text_index2, Link::new(text_long_index, link_finish, link_to));

        (false, middle_id)
    }

    fn update(&mut self, point: TriePoint, finish: TextIndex, text: &[u8]) -> TriePoint {
        let cap = self.capacity;
        let mut old_root_id = self.root_id;
        let mut cur_node_id = point.0;
        let mut cur_start = point.1;
        let mut split_result = self.test_and_split(point, finish, text);
        // println!("UPD1: i: {}, cur_node_id: {}, cur_start: {}", text[finish as usize] as char, cur_node_id, cur_start);
        // println!("UPD1: split_result: {}, {}", split_result.0, split_result.1);
        let index_finish = self.link_index(text, finish);

        while !split_result.0 {
            let new_node_id = self.nodes.push(Node::new(cap, None)) as NodeID;
            self.node_as_mut(split_result.1).link(index_finish, Link::new(finish, INFINITE, new_node_id));
            self.node_as_mut(new_node_id).id = new_node_id;

            if old_root_id != self.root_id {
                self.node_as_mut(old_root_id).suffix_link = Some(split_result.1);
            }
            old_root_id = split_result.1;

            let cur_node_id_suffix = self.node_as_ref(cur_node_id).suffix_link.unwrap();
            let can_result = self.canonize((cur_node_id_suffix, cur_start), finish, text);
            // println!("UPD2: i: {}, cur_node_suffix: {}, canonize: ({}, {})", text[finish as usize] as char,
                     // cur_node_id_suffix, can_result.0, can_result.1);
            cur_node_id = can_result.0;
            cur_start = can_result.1;

            split_result = self.test_and_split((cur_node_id, cur_start), finish, text);
            // println!("UPD3: i: {}, split_result: ({}, {})", text[finish as usize] as char, split_result.0, split_result.1);
        }
        if old_root_id != self.root_id {
            self.node_as_mut(old_root_id).suffix_link = Some(split_result.1);
        }
        (cur_node_id, cur_start)
    }

    fn contains(&self, text: &[u8], pattern: &[u8]) -> bool {
        let mut node_id = self.root_id;
        let mut start: TextIndex = 0;
        let mut end: TextIndex = 0;
        for i in 0..pattern.len() {
            if end == start {
                match self.link_as_ref(pattern, (node_id, i as TextIndex)) {
                    &None           => return false,
                    &Some(ref l)    => {
                        start = l.start;
                        end = l.start + 1;
                    },
                }
            } else {
                if pattern[i] != text[end as usize] { return false; }
                end += 1;
            }

            if end == self.link_as_ref(text, (node_id, start)).as_ref().unwrap().finish {
                node_id = self.link_as_ref(text, (node_id, start)).as_ref().unwrap().to;
                start = 0;
                end = 0;
            }
        }
        return true;
    }

    fn traverse_preorder_recursive<F>(&self, node_id: NodeID, lvl: usize, index: usize, func: &mut F)
    where F: FnMut(&Link, usize, usize)
    {
        let node = self.node_as_ref(node_id);

        for i in 0..self.capacity {
            match &node.links[i] {
                &Some(ref l) => {
                    func(&l, lvl, index);
                    self.traverse_preorder_recursive(l.to, lvl + 1, i, func);
                },
                &None    => continue,
            }
        }
    }

    pub fn traverse_preorder<F>(&self, func: &mut F)
    where F: FnMut(&Link, usize, usize)
    {
        self.traverse_preorder_recursive(self.root_id, 0, 0, func);
    }
}

type SuffixTrieUkkonen = SuffixTrie;

fn main() {
    let alphabet = b"az";
    let mut trie = SuffixTrieUkkonen::new(alphabet);

    // let text = b"xabxa";
    let text = b"xabxbcaxbcbasdbcacb";
    // let text = b"abcdef";
    // let text = b"acbacabcabca";
    trie.build(text);
    assert!(trie.contains(text, b"cbas"));
    assert!(trie.contains(text, b"xabx"));
    assert!(trie.contains(text, b"bcax"));
    assert!(trie.contains(text, b"axbc"));
    assert!(trie.contains(text, b"abxb"));
    assert!(!trie.contains(text, b"axxb"));

    println!("{}", String::from_utf8_lossy(text));
    // println!("{}", trie.nodes.slots.len());

    let len = text.len();
    let mut collect_result = |link: &Link, _lvl: usize, index: usize| {
        println!("link({}) to node {}: '{}'", (index as u8 + trie.min_symbol) as char, link.to,
                    String::from_utf8_lossy(&text[link.start as usize..min(link.finish as usize + 1, len)]));
    };
    trie.traverse_preorder(&mut collect_result);
}
