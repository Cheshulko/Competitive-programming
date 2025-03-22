// https://leetcode.com/problems/count-the-number-of-complete-components

struct Solution {}

impl Solution {
    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let adj = edges.into_iter().fold(vec![vec![]; n], |mut adj, edge| {
            let v = edge[0] as usize;
            let u = edge[1] as usize;

            adj[v].push(u);
            adj[u].push(v);

            adj
        });

        let mut ans = 0;
        let mut seen = vec![false; n];
        'out: for i in 0..n {
            if seen[i] {
                continue;
            }

            seen[i] = true;

            if adj[i].iter().any(|&to| seen[to]) {
                continue;
            }

            let mut all = vec![i]
                .into_iter()
                .chain(adj[i].iter().map(|x| *x))
                .collect::<Vec<_>>();

            all.sort_unstable();

            for &j in all.iter() {
                let mut all_j = vec![j]
                    .into_iter()
                    .chain(adj[j].iter().map(|x| *x))
                    .collect::<Vec<_>>();

                all_j.sort_unstable();

                if all_j != all {
                    continue 'out;
                }
            }

            ans += 1;
        }

        ans
    }
}
