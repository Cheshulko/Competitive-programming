// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree

struct Solution {}

impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let n = nums.len();

        let edges = edges.into_iter().fold(vec![vec![]; n], |mut edges, edge| {
            let u = edge[0] as usize;
            let v = edge[1] as usize;

            edges[v].push(u);
            edges[u].push(v);

            edges
        });

        fn dfs(
            cur: usize,
            p: i32,
            edges: &Vec<Vec<usize>>,
            nums: &Vec<i32>,
            in_subtree: &mut Vec<HashSet<usize>>,
            xor_in_subtree: &mut Vec<i32>,
        ) {
            in_subtree[cur].insert(cur);
            xor_in_subtree[cur] ^= nums[cur];

            for &to in edges[cur].iter() {
                if to as i32 == p {
                    continue;
                }
                dfs(to, cur as i32, edges, nums, in_subtree, xor_in_subtree);

                xor_in_subtree[cur] ^= xor_in_subtree[to];

                let to = in_subtree[to].clone();
                in_subtree[cur].extend(to);
            }
        }

        let mut in_subtree = vec![HashSet::new(); n];
        let mut xor_in_subtree = vec![0; n];
        dfs(0, -1, &edges, &nums, &mut in_subtree, &mut xor_in_subtree);

        let all = xor_in_subtree[0];
        let mut ans = i32::MAX;
        for v in 1..n {
            for u in 1..n {
                if v == u {
                    continue;
                }

                let (x1, x2, x3) = if in_subtree[v].contains(&u) {
                    let x1 = all ^ xor_in_subtree[v];
                    let x2 = xor_in_subtree[v] ^ xor_in_subtree[u];
                    let x3 = xor_in_subtree[u];

                    (x1, x2, x3)
                } else if in_subtree[u].contains(&v) {
                    let x1 = all ^ xor_in_subtree[u];
                    let x2 = xor_in_subtree[u] ^ xor_in_subtree[v];
                    let x3 = xor_in_subtree[v];

                    (x1, x2, x3)
                } else {
                    let x1 = xor_in_subtree[v];
                    let x2 = xor_in_subtree[u];
                    let x3 = all ^ x1 ^ x2;

                    (x1, x2, x3)
                };

                let d = x1.max(x2).max(x3) - x1.min(x2).min(x3);
                ans = ans.min(d);
            }
        }

        ans
    }
}
