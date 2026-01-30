// https://leetcode.com/problems/minimum-cost-to-convert-string-ii

mod cm_trie {
    #[derive(Clone, Debug)]
    struct Node<T> {
        // Maps character index -> index in self.t
        next: Vec<Option<usize>>,
        // Associated data (e.g., the string ID)
        data: Option<T>,
    }

    impl<T> Node<T> {
        pub fn new(alphabet_size: usize) -> Self {
            Node {
                next: vec![None; alphabet_size],
                data: None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Trie<T> {
        alphabet_size: usize,
        t: Vec<Node<T>>,
    }

    impl<T> Trie<T> {
        pub fn new(alphabet_size: usize) -> Self {
            let root = Node::new(alphabet_size);
            Trie {
                alphabet_size,
                t: vec![root],
            }
        }

        /// Insert a string and its associated value. Overwrites if exists.
        pub fn insert(&mut self, s: &[u8], value: T) {
            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                // Expand arena if path doesn't exist
                if self.t[v].next[idx].is_none() {
                    self.t[v].next[idx] = Some(self.t.len());
                    self.t.push(Node::new(self.alphabet_size));
                }
                v = self.t[v].next[idx].unwrap();
            }
            self.t[v].data = Some(value);
        }

        /// Retrieve a reference to the data associated with the full string s.
        pub fn get(&self, s: &[u8]) -> Option<&T> {
            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                if let Some(next_node) = self.t[v].next[idx] {
                    v = next_node;
                } else {
                    return None;
                }
            }
            self.t[v].data.as_ref()
        }

        /// Retrieve a mutable reference to the data.
        pub fn get_mut(&mut self, s: &[u8]) -> Option<&mut T> {
            let mut v = 0;
            for &ch in s {
                let idx = ch as usize;
                if let Some(next_node) = self.t[v].next[idx] {
                    v = next_node;
                } else {
                    return None;
                }
            }
            self.t[v].data.as_mut()
        }

        pub fn contains_key(&self, s: &[u8]) -> bool {
            self.get(s).is_some()
        }

        /// Returns the index of the root node (always 0).
        #[inline(always)]
        pub fn root(&self) -> usize {
            0
        }

        /// Moves from `node_idx` to the child corresponding to `ch`.
        /// Returns `Some(next_node_index)` if it exists, or `None`.
        /// Note: `ch` must be the mapped index (0..alphabet_size), not the raw ASCII if alphabet_size is small.
        #[inline(always)]
        pub fn step(&self, node_idx: usize, ch: u8) -> Option<usize> {
            // Safety: We assume caller passes valid char index for performance
            self.t.get(node_idx)?.next.get(ch as usize)?.clone()
        }

        /// Returns a reference to the data at the specific node index, if any.
        #[inline(always)]
        pub fn get_value_at(&self, node_idx: usize) -> Option<&T> {
            self.t.get(node_idx)?.data.as_ref()
        }
    }
}

mod cm_graph {
    pub fn floyd_warshall(dist: &mut Vec<Vec<i64>>) {
        let n = dist.len();

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i64::MAX && dist[k][j] != i64::MAX {
                        if dist[i][k] + dist[k][j] < dist[i][j] {
                            dist[i][j] = dist[i][k] + dist[k][j];
                        }
                    }
                }
            }
        }
    }
}

struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        use std::collections::{HashMap, HashSet};

        let source = source
            .into_bytes()
            .into_iter()
            .map(|b| b - b'a')
            .collect::<Vec<_>>();

        let target = target
            .into_bytes()
            .into_iter()
            .map(|b| b - b'a')
            .collect::<Vec<_>>();

        let original = original
            .into_iter()
            .map(|s| {
                s.into_bytes()
                    .into_iter()
                    .map(|b| b - b'a')
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let changed = changed
            .into_iter()
            .map(|s| {
                s.into_bytes()
                    .into_iter()
                    .map(|b| b - b'a')
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut trie = cm_trie::Trie::new(26);

        let mut un = 0;
        for s in original.iter().chain(changed.iter()) {
            if trie.contains_key(s) {
                continue;
            }

            trie.insert(s, un);
            un += 1;
        }

        let mut dist = vec![vec![i64::MAX; un]; un];
        for (i, from) in original.iter().enumerate() {
            let from = trie.get(from).copied().unwrap();
            let to = trie.get(&changed[i]).copied().unwrap();
            let cost = cost[i];

            dist[from][to] = dist[from][to].min(cost as i64);
        }

        cm_graph::floyd_warshall(&mut dist);

        let n = source.len();
        let mut dp = vec![i64::MAX; n + 1];
        dp[0] = 0;

        for i in 0..n {
            if dp[i] == i64::MAX {
                continue;
            }

            let mut from_node = trie.root();
            let mut to_node = trie.root();

            let mut same = true;
            for j in i..n {
                same = same && (source[j] == target[j]);
                if same {
                    dp[j + 1] = dp[j + 1].min(dp[i]);
                }

                if let (Some(from_node_new), Some(to_node_new)) = (
                    trie.step(from_node, source[j]),
                    trie.step(to_node, target[j]),
                ) {
                    from_node = from_node_new;
                    to_node = to_node_new;

                    if let (Some(&uid), Some(&vid)) =
                        (trie.get_value_at(from_node), trie.get_value_at(to_node))
                    {
                        if dist[uid][vid] != i64::MAX {
                            dp[j + 1] = dp[j + 1].min(dp[i] + dist[uid][vid]);
                        }
                    }
                } else {
                    break;
                }
            }
        }

        if dp[n] == i64::MAX {
            -1
        } else {
            dp[n]
        }
    }
}
