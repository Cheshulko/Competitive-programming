// https://leetcode.com/problems/count-prefix-and-suffix-pairs-ii

use std::collections::HashMap;

mod cm_trie {
    #[derive(Clone)]
    struct Node {
        next: Vec<Option<usize>>,
        cnt_words: usize,
    }

    impl Node {
        pub fn new(alphabet_size: usize) -> Self {
            Node {
                next: vec![None; alphabet_size],
                cnt_words: 0,
            }
        }
    }

    pub struct Trie {
        alphabet_size: usize,
        t: Vec<Node>,
    }

    impl Trie {
        pub fn new(alphabet_size: usize) -> Self {
            Trie {
                alphabet_size,
                t: vec![Node::new(alphabet_size); alphabet_size],
            }
        }
        /// Insert a string into the Trie
        /// - Time: O(|s|)
        pub fn insert(&mut self, s: &[usize]) {
            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    self.t[v].next[idx] = Some(self.t.len());
                    self.t.push(Node::new(self.alphabet_size));
                }
                v = self.t[v].next[idx].unwrap();
                self.t[v].cnt_words += 1;
            }
        }

        /// Find the number of times a string appears in the Trie
        /// - Time: O(|s|)
        pub fn find(&self, s: &[usize]) -> usize {
            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    return 0;
                }
                v = self.t[v].next[idx].unwrap();
            }
            self.t[v].cnt_words
        }
    }
}

struct Solution {}

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i64 {
        let mut map = HashMap::<(u8, u8), usize>::new();

        for word in words.iter() {
            for i in 0..word.len() {
                let word = word.as_bytes();

                let first = word[i] - b'a';
                let second = word[word.len() - 1 - i] - b'a';
                if !map.contains_key(&(first, second)) {
                    let ind = map.len();
                    map.insert((first, second), ind);
                }
            }
        }

        fn get_paired_word(word: &[u8], map: &HashMap<(u8, u8), usize>) -> Vec<usize> {
            let mut paired_word = vec![];
            for i in 0..word.len() {
                let first = word[i] - b'a';
                let second = word[word.len() - 1 - i] - b'a';

                paired_word.push(*map.get(&(first, second)).unwrap());
            }

            paired_word
        }

        let mut trie = cm_trie::Trie::new(map.len() + 1);
        let mut ans = 0;
        for word in words.iter().rev() {
            let paired_word = get_paired_word(word.as_bytes(), &map);

            ans += trie.find(&paired_word);

            trie.insert(&paired_word);
        }

        ans as i64
    }
}
