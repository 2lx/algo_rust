// https://neerc.ifmo.ru/wiki/index.php?title=Алгоритм_Укконена
use std::cmp::{min, max};

type NodeID = usize;

pub struct ArenaNode {
    data: (usize, usize),
    id: NodeID,
    parent: Option<NodeID>,
    children: Vec<Option<NodeID>>,
}

impl ArenaNode
{
    pub fn new(cap: usize, data: (usize, usize)) -> Self {
        Self{ id: 0,
              data: data,
              parent: None,
              // link: None,
              children: (0..cap).map(|_| None).collect::<Vec<Option<NodeID>>>()
        }
    }

    #[allow(dead_code)]
    pub fn child_id(&self, index: usize) -> &Option<NodeID> {
        &self.children[index]
    }

    #[allow(dead_code)]
    pub fn insert_child(&mut self, index: usize, node_id: NodeID) {
        self.children[index] = Some(node_id);
    }
}

pub struct ArenaSuffixTrie {
    nodes: Vec<Option<ArenaNode>>,
    root: NodeID,
    min_symbol: u8,
    capacity: usize,
}

impl ArenaSuffixTrie
{
    fn new(alphabet: &[u8]) -> Self {
        let (al_min, al_max) = alphabet.iter()
                .fold((255u8, 0u8), |(smin, smax), &u| (min(smin, u), max(smax, u)));
        let cap = (al_max - al_min + 1) as usize;

        let mut nodes = Vec::<Option<ArenaNode>>::new();
        nodes.push(Some(ArenaNode::new(cap, (0, 0))));
        Self {
                nodes: nodes,
                root: 0,
                min_symbol: al_min,
                capacity: cap,
        }
    }

    pub fn node_as_ref(&self, node_id: NodeID) -> & ArenaNode {
       self.nodes[node_id].as_ref().unwrap()
    }

    pub fn node_as_mut(&mut self, node_id: NodeID) -> &mut ArenaNode {
       self.nodes[node_id].as_mut().unwrap()
    }

    pub fn insert_node(&mut self, parent_id: NodeID, index: usize, node: ArenaNode) -> NodeID {
        self.nodes.push(Some(node));
        let new_node_id = self.nodes.len() - 1;

        self.node_as_mut(new_node_id).id = new_node_id;
        self.node_as_mut(new_node_id).parent = Some(parent_id);
        self.node_as_mut(parent_id).insert_child(index, new_node_id);
        new_node_id
    }

    pub fn replace_node(&mut self, parent_id: NodeID, child_index: usize, node: ArenaNode) -> ArenaNode {
        let child_id = self.node_as_ref(parent_id).child_id(child_index).unwrap();
        self.nodes.push(Some(node));
        let old_node = self.nodes.swap_remove(child_id);

        self.node_as_mut(child_id).id = child_id;
        self.node_as_mut(child_id).parent = Some(parent_id);
        old_node.unwrap()
    }

    pub fn take_node(&mut self, node_id: NodeID) -> ArenaNode {
        self.nodes[node_id].take().unwrap()
    }

    pub fn build(&mut self, text: &[u8]) {
        let len = text.len();
        for i in 0..len {
            self.build_suffix(text, i, len);
        }
    }

    pub fn build_suffix(&mut self, text: &[u8], start: usize, finish: usize) {
        let capacity = self.capacity;
        let mut cur_start = start;
        let mut cur_node_id = self.root;
        let mut cur_index = (text[cur_start] - self.min_symbol) as usize;

        while self.node_as_ref(cur_node_id).child_id(cur_index).is_some() {
            let child_id = self.node_as_ref(cur_node_id).child_id(cur_index).unwrap();;
            let (old_start, old_finish) = self.node_as_ref(child_id).data;
            let eq_len = text[old_start..min(old_finish + 1, finish)].iter()
                        .zip(text[cur_start..finish].iter().chain(b"$"))
                        .take_while(|&(&u1, &u2)| u1 == u2).count();

            if old_start + eq_len <= old_finish { // split at eq_len'th symbol
                let mut old_node = self.replace_node(cur_node_id, cur_index,
                                                   ArenaNode::new(capacity, (old_start, old_start + eq_len - 1)));
                self.node_as_mut(child_id).parent = Some(cur_node_id);
                old_node.data.0 = old_start + eq_len;

                let old_index = (text[old_start + eq_len] - self.min_symbol) as usize;
                self.insert_node(child_id, old_index, old_node);
            }
            cur_start += eq_len;
            cur_node_id = self.node_as_ref(cur_node_id).child_id(cur_index).unwrap();
            cur_index = if cur_start >= finish {self.capacity - 1} else {(text[cur_start] - self.min_symbol) as usize};
        }
        self.insert_node(cur_node_id, cur_index,
                          ArenaNode::new(capacity, (cur_start, finish)));
    }

    fn rec_traverse_preorder<F>(&self, node_id: NodeID, lvl: usize, index: usize, func: &mut F)
    where F: FnMut(&ArenaNode, usize, usize)
    {
        let node = self.node_as_ref(node_id);
        if lvl != 0 { func(&node, lvl, index); }

        for i in 0..node.children.len() {
            match node.children[i] {
                Some(n) => {
                    self.rec_traverse_preorder(n, lvl + 1, i, func);
                },
                None    => continue,
            }
        }
    }

    pub fn traverse_preorder<F>(&self, func: &mut F)
    where F: FnMut(&ArenaNode, usize, usize)
    {
        self.rec_traverse_preorder(self.root, 0, 0, func);
    }
}

type SuffixTrieUkkonen = ArenaSuffixTrie;

fn main() {
    let alphabet = b"az";
    let mut trie = SuffixTrieUkkonen::new(alphabet);

    // let text = b"xabxa";
    let text = b"acbacabcabca";
    trie.build(text);

    println!("{}", String::from_utf8_lossy(text));

    let mut collect_result = |node: &ArenaNode, lvl: usize, _index: usize| {
        let trail_symbol = if node.data.1 == text.len() {b'$'} else {text[node.data.1]};
        println!("id: {:>3}, lvl:{:>3}, parent: {:>3}, slice: '{}{}'",
                node.id,
                lvl,
                if node.parent.is_some() {node.parent.unwrap()} else {0},
                // if node.link.is_some() {node.link.unwrap()} else {0},
                String::from_utf8_lossy(&text[node.data.0..node.data.1]),
                trail_symbol as char,
                 );
    };
    trie.traverse_preorder(&mut collect_result);
}
