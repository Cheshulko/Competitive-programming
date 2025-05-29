// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii

struct Solution {}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let edges1 = build_edges(edges1);

        let m = edges2.len() + 1;
        let edges2 = build_edges(edges2);

        fn build_edges(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
            let l = edges.len() + 1;

            edges.into_iter().fold(vec![vec![]; l], |mut edges, edge| {
                let v = edge[0] as usize;
                let u = edge[1] as usize;
                edges[v].push(u);
                edges[u].push(v);

                edges
            })
        }

        fn dfs_count_1(cur: usize, par: i32, edges: &Vec<Vec<usize>>, dp: &mut Vec<[i32; 2]>) {
            dp[cur][0] += 1;

            for &to in edges[cur].iter() {
                if to as i32 == par {
                    continue;
                }

                dfs_count_1(to, cur as i32, edges, dp);
                dp[cur][0] += dp[to][1];
                dp[cur][1] += dp[to][0];
            }
        }

        fn dfs_count_2(cur: usize, par: i32, edges: &Vec<Vec<usize>>, dp: &mut Vec<[i32; 2]>) {
            if par != -1 {
                let par = par as usize;

                assert!(dp[par][0] >= dp[cur][1]);
                let par_0 = dp[par][0] - dp[cur][1];

                assert!(dp[par][1] >= dp[cur][0]);
                let par_1 = dp[par][1] - dp[cur][0];

                dp[cur][0] += par_1;
                dp[cur][1] += par_0;
            }

            for &to in edges[cur].iter() {
                if to as i32 == par {
                    continue;
                }

                dfs_count_2(to, cur as i32, edges, dp);
            }
        }

        let mut dp2 = vec![[0, 0]; m];
        dfs_count_1(0, -1, &edges2, &mut dp2);
        dfs_count_2(0, -1, &edges2, &mut dp2);

        let ma1 = dp2
            .into_iter()
            .max_by_key(|d| d[1])
            .map(|d| d[1])
            .unwrap_or(0);

        let mut dp1 = vec![[0, 0]; n];
        dfs_count_1(0, -1, &edges1, &mut dp1);
        dfs_count_2(0, -1, &edges1, &mut dp1);

        dp1.into_iter().map(|d| d[0] + ma1).collect()
    }
}
