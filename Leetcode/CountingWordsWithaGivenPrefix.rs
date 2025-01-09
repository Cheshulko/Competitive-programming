// https://leetcode.com/problems/counting-words-with-a-given-prefix

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
        pub fn insert(&mut self, s: &[u8]) {
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
        pub fn find(&self, s: &[u8]) -> usize {
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
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut trie = cm_trie::Trie::new(26);

        for word in words.into_iter() {
            let word = word
                .into_bytes()
                .into_iter()
                .map(|c| c - b'a')
                .collect::<Vec<_>>();

            trie.insert(&word);
        }

        let pref = pref
            .into_bytes()
            .into_iter()
            .map(|c| c - b'a')
            .collect::<Vec<_>>();

        trie.find(&pref) as i32
    }
}
