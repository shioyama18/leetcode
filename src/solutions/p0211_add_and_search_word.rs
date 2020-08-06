use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

struct WordDictionary {
    trie: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            trie: Default::default(),
        }
    }

    /** Adds a word into the data structure. */
    fn add_word(&mut self, word: String) {
        let mut node = &mut self.trie;
        for c in word.chars() {
            node = node.children.entry(c).or_insert(Default::default());
        }
        node.is_word = true;
    }

    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        Self::search_trie(&word, &self.trie)
    }

    fn search_trie(word: &str, node: &TrieNode) -> bool {
        let mut node = node;
        for (i, c) in word.chars().enumerate() {
            if node.children.contains_key(&c) {
                node = &node.children[&c];
            } else {
                if c == '.' {
                    for k in node.children.keys() {
                        let child = &node.children[&k];
                        if Self::search_trie(&word[i + 1..], child) {
                            return true;
                        }
                    }
                }

                return false;
            }
        }
        node.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_list() {
        let mut word_list = WordDictionary::new();
        word_list.add_word("bad".to_string());
        word_list.add_word("mad".to_string());
        word_list.add_word("pad".to_string());

        assert!(word_list.search("bad".to_string()));
        assert!(word_list.search(".ad".to_string()));
        assert!(!word_list.search("bat".to_string()));
    }
}
