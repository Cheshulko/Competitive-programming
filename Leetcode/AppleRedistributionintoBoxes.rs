// https://leetcode.com/problems/apple-redistribution-into-boxes

struct Solution {}

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let sum = apple.iter().sum::<i32>();

        capacity.sort_unstable();

        let mut ans = 0;
        let mut have = 0;
        for c in capacity.into_iter().rev() {
            have += c;
            ans += 1;

            if have >= sum {
                return ans;
            }
        }

        unreachable!()
    }
}
