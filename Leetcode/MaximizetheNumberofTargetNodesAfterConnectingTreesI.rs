// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i

struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        fn dfs_count(
            cur: usize,
            par: i32,
            depth: usize,
            edges: &Vec<Vec<usize>>,
            counts: &mut Vec<usize>,
        ) {
            counts[depth] += 1;

            for &to in edges[cur].iter() {
                if to as i32 == par {
                    continue;
                }

                dfs_count(to, cur as i32, depth + 1, edges, counts);
            }
        }

        let k = k as usize;
        let n = edges1.len() + 1;
        let m = edges2.len() + 1;
        let edges1 = edges1.into_iter().fold(vec![vec![]; n], |mut edges, edge| {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            edges[v].push(u);
            edges[u].push(v);

            edges
        });
        let edges2 = edges2.into_iter().fold(vec![vec![]; m], |mut edges, edge| {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            edges[v].push(u);
            edges[u].push(v);

            edges
        });

        let mut ma = 0;
        for i in 0..m {
            let mut m_counts = vec![0; m + 1];
            dfs_count(i, -1, 1, &edges2, &mut m_counts);

            let mut maybe_ma = 0;
            for depth in 1..=k.min(m) {
                maybe_ma += m_counts[depth];
            }
            ma = ma.max(maybe_ma);
        }

        let mut ans = vec![0; n];
        for i in 0..n {
            let mut m_counts = vec![0; n + 1];
            dfs_count(i, -1, 0, &edges1, &mut m_counts);

            let mut n_ma = 0;
            for depth in 0..=k.min(n) {
                n_ma += m_counts[depth];
            }

            ans[i] = (n_ma + ma) as i32;
        }

        ans
    }
}
