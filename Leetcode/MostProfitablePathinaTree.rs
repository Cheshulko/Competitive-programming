// https://leetcode.com/problems/most-profitable-path-in-a-tree

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, bob: i32, amount: Vec<i32>) -> i32 {
        let n = 1 + edges.iter().map(|v| v[0].max(v[1])).max().unwrap() as usize;
        let bob = bob as usize;

        let mut adj = vec![vec![]; n];
        for edge in edges.into_iter() {
            adj[edge[0] as usize].push(edge[1] as usize);
            adj[edge[1] as usize].push(edge[0] as usize);
        }

        fn dfs(cur: usize, p: i32, adj: &Vec<Vec<usize>>, t: usize, time: &mut Vec<usize>) {
            time[cur] = t;

            for &to in adj[cur].iter() {
                if to as i32 != p {
                    dfs(to, cur as i32, adj, t + 1, time);
                }
            }
        }

        fn dfs_bob(
            cur: usize,
            adj: &Vec<Vec<usize>>,
            t: usize,
            time: &Vec<usize>,
            time_bob: &mut Vec<usize>,
        ) {
            time_bob[cur] = t;

            for &to in adj[cur].iter() {
                if time[to] < time[cur] {
                    dfs_bob(to, adj, t + 1, time, time_bob);
                }
            }
        }

        fn dfs_alice(
            cur: usize,
            p: i32,
            t: usize,
            sum: i64,
            adj: &Vec<Vec<usize>>,
            amount: &Vec<i32>,
            time_bob: &Vec<usize>,
            ans: &mut i64,
        ) {
            let cur_sum = sum
                + match time_bob[cur].cmp(&t) {
                    Ordering::Less => 0,
                    Ordering::Equal => amount[cur] as i64 / 2,
                    Ordering::Greater => amount[cur] as i64,
                };

            if adj[cur].len() == 1 && adj[cur][0] as i32 == p {
                *ans = (*ans).max(cur_sum);

                return;
            }

            for &to in adj[cur].iter() {
                if to as i32 != p {
                    dfs_alice(to, cur as i32, t + 1, cur_sum, adj, amount, time_bob, ans);
                }
            }
        }

        let mut time = vec![0; n];
        dfs(0, -1, &adj, 0, &mut time);

        let mut time_bob = vec![usize::MAX; n];
        dfs_bob(bob, &adj, 0, &time, &mut time_bob);

        let mut ans = i64::MIN;
        dfs_alice(0, -1, 0, 0, &adj, &amount, &time_bob, &mut ans);

        ans as i32
    }
}
