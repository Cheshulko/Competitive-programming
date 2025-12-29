// https://leetcode.com/problems/pyramid-transition-matrix

use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn get(
        dp: &mut HashMap<usize, bool>,
        state: &mut Vec<usize>,
        row: &[Vec<usize>],
        allowed: &Vec<Vec<usize>>,
    ) -> bool {
        if row.is_empty() {
            return Solution::check(dp, state.to_vec(), allowed);
        }

        let mut can = false;
        for i in 0..row[0].len() {
            state.push(row[0][i]);
            can = can || Self::get(dp, state, &row[1..], allowed);
            state.pop();
        }

        can
    }

    fn check(dp: &mut HashMap<usize, bool>, row: Vec<usize>, allowed: &Vec<Vec<usize>>) -> bool {
        let n = row.len();

        if n == 1 {
            return true;
        }

        let mut hash = 0;
        for i in 0..n {
            hash = hash * 10 + row[i];
        }

        if let Some(&res) = dp.get(&hash) {
            return res;
        }

        let mut can = vec![vec![]; n - 1];
        for i in 0..n - 1 {
            for a in allowed.iter() {
                if a[0] == row[i] && a[1] == row[i + 1] {
                    can[i].push(a[2]);
                }
            }
        }

        let mut state = vec![];
        let can = Self::get(dp, &mut state, &can, allowed);

        dp.insert(hash, can);

        return can;
    }

    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let bottom = bottom
            .into_bytes()
            .into_iter()
            .map(|b| (b - b'a') as usize)
            .collect::<Vec<_>>();

        let allowed = allowed
            .into_iter()
            .map(|l| {
                l.into_bytes()
                    .into_iter()
                    .map(|b| (b - b'a') as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut dp = HashMap::new();

        return Self::check(&mut dp, bottom, &allowed);
    }
}
