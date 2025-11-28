// https://leetcode.com/problems/maximum-number-of-k-divisible-components

struct Solution {}

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        fn dfs(
            cur: usize,
            p: i32,
            edges: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i64,
            ans: &mut usize,
        ) -> i64 {
            let mut sum = 0;
            for &to in edges[cur].iter() {
                if to as i32 == p {
                    continue;
                }

                let to_sum = dfs(to, cur as i32, edges, values, k, ans);
                if to_sum % k == 0 {
                    *ans += 1;
                } else {
                    sum += to_sum;
                }
            }

            sum + values[cur] as i64
        }

        let n = n as usize;
        let k = k as i64;
        let edges = edges.into_iter().fold(vec![vec![]; n], |mut v, edge| {
            let a = edge[0] as usize;
            let b = edge[1] as usize;

            v[a].push(b);
            v[b].push(a);
            v
        });

        let mut ans = 0;
        let sum = dfs(0, -1, &edges, &values, k, &mut ans);

        1 + if sum % k == 0 { ans as i32 } else { 0 }
    }
}
