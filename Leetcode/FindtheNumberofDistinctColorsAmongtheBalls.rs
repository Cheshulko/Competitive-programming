// https://leetcode.com/problems/find-the-number-of-distinct-colors-among-the-balls

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn query_results(limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![];

        let mut positions: HashMap<i32, i32> = HashMap::new();
        let mut colors: HashMap<i32, i32> = HashMap::new();
        let mut diff = 0;
        for q in queries.into_iter() {
            let (p, c) = (q[0], q[1]);

            if let Some(&prev_c) = positions.get(&p) {
                let x = colors.entry(prev_c).or_default();
                *x -= 1;
                if *x == 0 {
                    diff -= 1;
                }
            }
            positions.insert(p, c);

            let x = colors.entry(c).or_default();
            if *x == 0 {
                diff += 1;
            }
            *x += 1;

            ans.push(diff);
        }

        ans
    }
}
