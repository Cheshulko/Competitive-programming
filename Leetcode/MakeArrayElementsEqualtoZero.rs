// https://leetcode.com/problems/make-array-elements-equal-to-zero

struct Solution {}

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let s1 = nums.iter().sum::<i32>();

        let mut s2 = 0;
        let mut ans = 0;
        for num in nums {
            s2 += num;
            if num == 0 && 2 * s2 == s1 {
                ans += 2;
            } else if num == 0 && s1.abs_diff(2 * s2) == 1 {
                ans += 1;
            }
        }

        ans
    }
}
