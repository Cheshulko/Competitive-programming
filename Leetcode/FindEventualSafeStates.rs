// https://leetcode.com/problems/find-eventual-safe-states

struct Solution {}

impl Solution {
    fn dfs(
        cur: usize,
        graph: &Vec<Vec<i32>>,
        path: &mut Vec<usize>,
        used: &mut Vec<bool>,
        bad: &mut Vec<bool>,
    ) {
        used[cur] = true;
        path.push(cur);

        for to in &graph[cur] {
            let to = *to as usize;
            if !used[to] {
                Solution::dfs(to, graph, path, used, bad);
            } else {
                if path.contains(&to) || bad[to] {
                    for i in 0..path.len() {
                        bad[path[i]] = true;
                    }
                }
            }
        }

        path.pop();
    }

    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let n = graph.len();
        let mut used = vec![false; n];
        let mut path = vec![];
        let mut bad = vec![false; n];

        for i in 0..n {
            if !used[i] {
                Solution::dfs(i, &graph, &mut path, &mut used, &mut bad);
            }
        }

        bad.into_iter()
            .enumerate()
            .filter_map(|(a, b)| if !b { Some(a as i32) } else { None })
            .collect::<Vec<_>>()
    }
}
