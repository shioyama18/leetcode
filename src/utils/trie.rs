use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct TrieNode {
    links: Vec<Option<Rc<RefCell<Self>>>>,
    is_end: bool,
}

impl TrieNode {
    const R: usize = 26;

    fn new() -> Self {
        TrieNode {
            links: vec![None; Self::R],
            is_end: false,
        }
    }

    fn contains_key(&self, c: char) -> bool {
        self.links[c as usize - 'a' as usize].is_some()
    }

    fn get(&self, c: char) -> Rc<RefCell<Self>> {
        Rc::clone(self.links[c as usize - 'a' as usize].as_ref().unwrap())
    }

    fn put(&mut self, c: char, node: TrieNode) {
        self.links[c as usize - 'a' as usize] = Some(Rc::new(RefCell::new(node)));
    }

    fn set_end(&mut self) {
        self.is_end = true;
    }

    fn is_end(&self) -> bool {
        self.is_end
    }
}

pub struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode::new())),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut p = Rc::clone(&self.root);
        for c in word.chars() {
            if !p.borrow().contains_key(c) {
                p.borrow_mut().put(c, TrieNode::new());
            }
            let next = p.borrow().get(c);
            p = Rc::clone(&next);
        }
        p.borrow_mut().set_end();
    }

    pub fn search(&self, word: String) -> bool {
        let mut p = Rc::clone(&self.root);
        for c in word.chars() {
            if !p.borrow().contains_key(c) {
                return false;
            }
            let next = p.borrow().get(c);
            p = Rc::clone(&next);
        }

        if p.borrow().is_end() {
            return true;
        }

        false
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut p = Rc::clone(&self.root);
        for c in prefix.chars() {
            if !p.borrow().contains_key(c) {
                return false;
            }
            let next = p.borrow().get(c);
            p = Rc::clone(&next);
        }
        true
    }
}
