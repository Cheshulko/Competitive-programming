mod cm_trie {
    #[derive(Clone)]
    struct Node {
        next: Vec<Option<usize>>,
        cnt_words: usize,
        cnt_prefixes: usize,
    }

    impl Node {
        pub fn new(alphabet_size: usize) -> Self {
            Node {
                next: vec![None; alphabet_size],
                cnt_words: 0,
                cnt_prefixes: 0,
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
                self.t[v].cnt_prefixes += 1;
            }
            self.t[v].cnt_words += 1;
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

        /// Find the number of times a prefix appears in the Trie
        /// - Time: O(|s|)
        pub fn find_prefix(&self, s: &[u8]) -> usize {
            let mut v = 0;
            let mut ans = 0;
            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    return 0;
                }
                v = self.t[v].next[idx].unwrap();
                ans += self.t[v].cnt_prefixes;
            }
            ans
        }

        /// Find the prefix for the given input
        /// - Time: O(|s|)
        pub fn longest_prefix(&self, s: &[u8]) -> usize {
            let mut v = 0;
            let mut len = 0;
            for &ch in s {
                let idx = ch as usize;
                if self.t[v].next[idx].is_none() {
                    return len;
                }
                len += 1;
                v = self.t[v].next[idx].unwrap();
            }
            len
        }
    }
}

struct Solution {}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let words = words
            .into_iter()
            .map(|word| {
                word.into_bytes()
                    .into_iter()
                    .map(|c| c - b'a')
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut trie = cm_trie::Trie::new(26);
        for word in words.iter() {
            trie.insert(word);
        }

        words
            .into_iter()
            .map(|word| trie.find_prefix(&word) as i32)
            .collect::<Vec<_>>()
    }
}
