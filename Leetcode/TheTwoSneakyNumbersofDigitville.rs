// https://leetcode.com/problems/the-two-sneaky-numbers-of-digitville

struct Solution {}

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; 101];

        let mut ans = vec![];
        for num in nums {
            if seen[num as usize] {
                ans.push(num);
            } else {
                seen[num as usize] = true;
            }
        }

        ans
    }
}
