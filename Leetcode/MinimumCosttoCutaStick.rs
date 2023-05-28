// https://leetcode.com/problems/minimum-cost-to-cut-a-stick

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        let mut dp: HashMap<(i32, i32), i32> = HashMap::new();
        cuts.sort();

        fn go(s: i32, e: i32, dp: &mut HashMap<(i32, i32), i32>, cuts: &[i32]) -> i32 {
            if dp.contains_key(&(s, e)) {
                return *dp.get(&(s, e)).unwrap();
            }

            if cuts.len() == 0 {
                dp.insert((s, e), 0);
                return 0;
            }

            let mut ans = i32::MAX;
            for cut_ind in 0..cuts.len() {
                let cuts_left = &cuts[..cut_ind];

                let cuts_right = if cut_ind == cuts.len() - 1 {
                    &[]
                } else {
                    &cuts[(cut_ind + 1)..]
                };

                let left = go(s, cuts[cut_ind], dp, cuts_left);
                let right = go(cuts[cut_ind], e, dp, cuts_right);

                ans = ans.min((e - s) as i32 + left + right);
            }

            dp.insert((s, e), ans);
            return ans;
        }

        go(0, n, &mut dp, &cuts)
    }
}
