// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field

struct Solution;

impl Solution {
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        h_fences.extend([m].into_iter());
        h_fences.sort_unstable();

        v_fences.extend([n].into_iter());
        v_fences.sort_unstable();

        use std::collections::HashSet;

        let mut h_possible = HashSet::new();
        for i in 0..h_fences.len() {
            h_possible.insert(h_fences[i] - 1);

            for j in 0..i {
                h_possible.insert(h_fences[i] - h_fences[j]);
            }
        }

        let mut v_possible = HashSet::new();
        for i in 0..v_fences.len() {
            v_possible.insert(v_fences[i] - 1);

            for j in 0..i {
                v_possible.insert(v_fences[i] - v_fences[j]);
            }
        }

        if let Some(best) = h_possible.intersection(&v_possible).max().copied() {
            const M: i64 = 1_000_000_000 + 7;

            let best = best as i64;

            (best * best % M) as i32
        } else {
            -1
        }
    }
}
