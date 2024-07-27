// https://leetcode.com/problems/minimum-cost-to-convert-string-i

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let original = original
            .into_iter()
            .map(|x| (x as u8 - b'a') as usize)
            .collect::<Vec<_>>();
        let changed = changed
            .into_iter()
            .map(|x| (x as u8 - b'a') as usize)
            .collect::<Vec<_>>();
        let cost = cost.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let mut mtx = vec![vec![i64::MAX; 26]; 26];
        for i in 0..26 {
            mtx[i][i] = 0;
        }

        for i in 0..original.len() {
            let cur = original[i];
            let mut cost_sum = 0;

            let mut q = VecDeque::<usize>::new();
            q.push_back(cur);
            while let Some(cur) = q.pop_front() {
                for j in 0..original.len() {
                    if original[j] == cur {
                        let to = changed[j];
                        if mtx[original[i]][to] > mtx[original[i]][cur] + cost[j] {
                            mtx[original[i]][to] = mtx[original[i]][cur] + cost[j];
                            q.push_back(to);
                        }
                    }
                }
            }
        }

        let source = source.into_bytes();
        let target = target.into_bytes();

        let mut ans = 0;
        for i in 0..source.len() {
            let from = (source[i] - b'a') as usize;
            let to = (target[i] - b'a') as usize;

            if mtx[from][to] == i64::MAX {
                ans = -1;
                break;
            } else {
                ans += mtx[from][to];
            }
        }

        ans
    }
}
