use std::collections::HashMap;

#[derive(Debug, Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

#[derive(Debug, Default)]
struct StreamChecker {
    trie: TrieNode,
    stream: Vec<char>,
}

impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let mut trie = TrieNode::default();

        for word in words.into_iter() {
            let mut node = &mut trie;
            for c in word.chars().rev() {
                node = node.children.entry(c).or_default();
            }
            node.is_end = true;
        }

        Self {
            trie,
            ..Default::default()
        }
    }

    fn query(&mut self, letter: char) -> bool {
        self.stream.push(letter);
        let mut node = &self.trie;

        for c in self.stream.iter().rev() {
            if let Some(next) = node.children.get(&c) {
                if next.is_end {
                    return true;
                }
                node = next;
            } else {
                return false;
            }
        }

        false
    }
}
