// https://leetcode.com/problems/find-the-maximum-sum-of-node-values

use std::mem::swap;

struct Solution {}

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let k = k as i64;

        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let edges = edges.into_iter().fold(vec![vec![]; nums.len()], |mut edges, edge| {
            let v = edge[0] as usize;
            let u = edge[1] as usize;
            edges[v].push(u);
            edges[u].push(v);

            edges
        });

        let root = edges
            .iter()
            .enumerate()
            .find(|(_, edges)| edges.len() == 1)
            .unwrap()
            .0;

        #[rustfmt::skip]
        fn dfs(cur: usize, par: i32, k: i64, nums: &Vec<i64>, edges: &Vec<Vec<usize>>) -> [i64; 2] {
            let mut ldp  = [0, i64::MIN];
            let mut nldp = [0, 0];

            for &to in edges[cur].iter() {
                if to as i32 == par {
                    continue;
                }

                let res = dfs(to, cur as i32, k, nums, edges);

                nldp[0] = (
                    ldp[0] + nums[to] + res[0]).max(
                    ldp[0] + (nums[to] ^ k) + res[1]).max(
                    ldp[1] + (nums[to] ^ k) + res[0]).max(
                    ldp[1] + nums[to] + res[1]);
                
                nldp[1] = (
                    ldp[1] + nums[to] + res[0]).max(
                    ldp[1] + (nums[to] ^ k) + res[1]).max(
                    ldp[0] + (nums[to] ^ k) + res[0]).max(
                    ldp[0] + nums[to] + res[1]);
                    
                swap(&mut ldp, &mut nldp);
            }

            ldp
        }

        let res = dfs(root, -1, k, &nums, &edges);

        (res[0] + nums[root]).max(res[1] + (nums[root] ^ k))
    }
}