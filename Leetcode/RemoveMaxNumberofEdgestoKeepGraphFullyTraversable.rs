// https://leetcode.com/problems/remove-max-number-of-edges-to-keep-graph-fully-traversable

mod graph {
    use std::{
        cmp::Reverse,
        collections::{BTreeMap, BinaryHeap},
    };

    pub struct Prim {
        pub n: i32,
        pub graph: BTreeMap<i32, BTreeMap<i32, i32>>,
        pub mst: BTreeMap<i32, BTreeMap<i32, i32>>,
    }

    impl Prim {
        pub fn new(n: i32) -> Self {
            Prim {
                n,
                graph: BTreeMap::new(),
                mst: BTreeMap::new(),
            }
        }

        pub fn add_edge(&mut self, v1: i32, v2: i32, c: i32) {
            let mut need_rewrite = true;

            if let Some(from) = self.graph.get(&v1) {
                if let Some(len) = from.get(&v2) {
                    if len < &c {
                        need_rewrite = false;
                    }
                }
            }

            if need_rewrite {
                self.graph
                    .entry(v1)
                    .or_insert_with(BTreeMap::new)
                    .insert(v2, c);
                self.graph
                    .entry(v2)
                    .or_insert_with(BTreeMap::new)
                    .insert(v1, c);
            }
        }

        pub fn prim(&mut self) -> bool {
            match self.graph.keys().next() {
                Some(v) => {
                    self.prim_with_start(*v);
                    self.check_connectivity()
                }
                None => false,
            }
        }

        fn check_connectivity(&self) -> bool {
            !(1..=self.n).any(|node| !self.mst.contains_key(&node))
        }

        fn add_edge_mst(&mut self, v1: i32, v2: i32, c: i32) {
            self.mst
                .entry(v1)
                .or_insert_with(BTreeMap::new)
                .insert(v2, c);
            self.mst
                .entry(v2)
                .or_insert_with(BTreeMap::new)
                .insert(v1, c);
        }

        fn prim_with_start(&mut self, start: i32) {
            let mut prio = BinaryHeap::new();

            self.mst.insert(start, BTreeMap::new());

            for (v, c) in &self.graph[&start] {
                prio.push(Reverse((*c, *v, start)));
            }

            while let Some(Reverse((dist, t, prev))) = prio.pop() {
                if self.mst.contains_key(&t) {
                    continue;
                }

                self.add_edge_mst(prev, t, dist);

                for (v, c) in &self.graph[&t] {
                    if !self.mst.contains_key(v) {
                        prio.push(Reverse((*c, *v, t)));
                    }
                }
            }
        }
    }
}

struct Solution {}

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut alice = graph::Prim::new(n);
        let mut bob = graph::Prim::new(n);

        edges.iter().for_each(|edge| match edge[0] {
            1 => alice.add_edge(edge[1], edge[2], 2),
            2 => bob.add_edge(edge[1], edge[2], 2),
            3 => {
                // Add common edges with weight of 1
                alice.add_edge(edge[1], edge[2], 1);
                bob.add_edge(edge[1], edge[2], 1);
            }
            _ => {}
        });

        let start_edges_cnt = edges.len();

        let is_alice_connected = alice.prim();
        let is_bob_connected = bob.prim();

        if is_alice_connected && is_bob_connected {
            let mut final_edges_cnt = 0;

            // double cnt, because edges are bidirected
            let (alice_edges_cnt, alice_common_edges_cnt) =
                alice.mst.iter().fold((0, 0), |cur, (_, edges)| {
                    (
                        cur.0 + edges.len(),
                        cur.1 + edges.iter().filter(|x| x.1 == &1).count(),
                    )
                });

            // double cnt, because edges are bidirected
            let (bob_edges_cnt, bob_common_edges_cnt) =
                bob.mst.iter().fold((0, 0), |cur, (_, edges)| {
                    (
                        cur.0 + edges.len(),
                        cur.1 + edges.iter().filter(|x| x.1 == &1).count(),
                    )
                });

            assert!(alice_common_edges_cnt == bob_common_edges_cnt);
            final_edges_cnt = (alice_edges_cnt + bob_edges_cnt - alice_common_edges_cnt) / 2;

            (start_edges_cnt - final_edges_cnt) as i32
        } else {
            -1
        }
    }
}
