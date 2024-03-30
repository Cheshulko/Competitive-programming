// https://leetcode.com/problems/minimum-levels-to-gain-more-points

struct Solution {}

impl Solution {
    pub fn minimum_levels(possible: Vec<i32>) -> i32 {
        let n = possible.len();

        let mut sum = possible
            .iter()
            .map(|x| if x == &1 { 1 } else { -1 })
            .sum::<i32>();
        let mut cur = 0;

        for i in 0..(n - 1) {
            let add = if possible[i] == 1 { 1 } else { -1 };
            cur += add;
            sum -= add;

            if cur > sum {
                return i as i32 + 1;
            }
        }

        return -1;
    }
}
