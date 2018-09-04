pub struct TrieNode<T> {
    #[allow(dead_code)]
    pub data: T,
    children: Vec<Option<TrieNode<T>>>,
}

impl<T> TrieNode<T> {
    pub fn new(cap: usize, data: T) -> Self {
        Self{ data: data,
              children: (0..cap).map(|_| None).collect::<Vec<Option<TrieNode<T>>>>()
        }
    }

    pub fn child_as_ref(&self, index: usize) -> Option<&TrieNode<T>> {
        self.children[index].as_ref()
    }

    pub fn child_as_mut(&mut self, index: usize) -> Option<&mut TrieNode<T>> {
        self.children[index].as_mut()
    }

    #[allow(dead_code)]
    pub fn insert_child(&mut self, index: usize, node: TrieNode<T>) {
        self.children[index] = Some(node);
    }

    #[allow(dead_code)]
    pub fn take_child(&mut self, index: usize) -> TrieNode<T> {
        self.children[index].take().unwrap()
    }

    #[allow(dead_code)]
    pub fn traverse_preorder<F>(&self, lvl: usize, index: usize, func: &mut F)
    where F: FnMut(&T, usize, usize)
    {
        if lvl != 0 { func(&self.data, lvl, index); }

        for i in 0..self.children.len() {
            match &self.children[i] {
                &Some(ref n) => { n.traverse_preorder(lvl + 1, i, func); },
                &None        => continue,
            }
        }
    }

    #[allow(dead_code)]
    pub fn traverse_postorder<F>(&self, lvl: usize, index: usize, func: &mut F)
    where F: FnMut(&T, usize, usize)
    {
        for i in 0..self.children.len() {
            match &self.children[i] {
                &Some(ref n) => { n.traverse_postorder(lvl + 1, i, func); },
                &None        => continue,
            }
        }

        if lvl != 0 { func(&self.data, lvl, index); }
    }
}

