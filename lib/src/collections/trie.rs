use std::collections::HashMap;


struct TrieNode {
    word: Option<String>,
    children: HashMap<char, Box<TrieNode>>
}

impl TrieNode {
    pub fn new() -> Self {
        TrieNode { word: None, children: HashMap::new() }
    }
}

struct Trie {
    root: Box<TrieNode>
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Box::new(TrieNode::new()) }
    }

    pub fn insert(&mut self, value: &str) {
        let mut current = &mut self.root;
        for c in value.chars()
        {
            if current.children.contains_key(&c)
            {
                current = current.children.get_mut(&c).unwrap();
            } else {
                let new_node = Box::new(TrieNode::new());
                current.children.insert(c, new_node);
                current = current.children.get_mut(&c).unwrap();
            }
        }

        current.word = Some(String::from(value));
    }

    pub fn contains(&self, value: &str) -> bool {
        let mut current = &self.root;
        for c in value.chars()
        {
            if current.children.contains_key(&c)
            {
                current = current.children.get(&c).unwrap();
            } else {
                return false;
            }
        }

        if let Some(word) = &current.word { return word.eq_ignore_ascii_case(value); }
        else { false }
    }
}



#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn trie_basics()
    {
        let mut my_trie = Trie::new();

        my_trie.insert("test");
        assert_eq!(my_trie.contains("test"), true);
        my_trie.insert("tester");
        assert_eq!(my_trie.contains("test"), true);
        assert_eq!(my_trie.contains("tester"), true);
    }
}