// https://leetcode.com/problems/evaluate-division

use std::collections::{BTreeMap, HashSet};

struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: BTreeMap<String, Vec<(String, f64)>> = BTreeMap::new();

        equations.iter().zip(values.iter()).for_each(|(eq, w)| {
            let from = &eq[0];
            let to = &eq[1];

            graph
                .entry(from.clone())
                .or_insert(vec![])
                .push((to.clone(), *w));

            graph
                .entry(to.clone())
                .or_insert(vec![])
                .push((from.clone(), 1.0 / *w));
        });

        queries
            .iter()
            .map(|q| {
                let from = &q[0];
                let to = &q[1];
                let mut used: HashSet<String> = HashSet::new();

                if graph.contains_key(from) {
                    Solution::dfs(&graph, &mut used, from, to).unwrap_or(-1.0)
                } else {
                    -1.0
                }
            })
            .collect()
    }

    fn dfs(
        graph: &BTreeMap<String, Vec<(String, f64)>>,
        used: &mut HashSet<String>,
        cur: &String,
        target: &String,
    ) -> Option<f64> {
        used.insert(cur.clone());

        if cur == target {
            return Some(1.0);
        } else {
            if let Some(edges) = graph.get(cur) {
                for (to, w) in edges {
                    if !used.contains(to) {
                        if let Some(res) = Solution::dfs(graph, used, to, target) {
                            return Some(res * w);
                        }
                    }
                }
            }
            return None;
        }
    }
}
