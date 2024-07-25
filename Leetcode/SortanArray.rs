//  https://leetcode.com/problems/sort-an-array

struct Solution {}

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        const MAX: usize = 2 * 100_000;
        let mut cnt = vec![0; 2 * MAX + 1];

        for num in nums.into_iter() {
            cnt[(num + MAX as i32) as usize] += 1;
        }

        let mut ans = vec![];
        for i in 0..=2 * MAX {
            for _ in 0..cnt[i] {
                ans.push(i as i32 - MAX as i32);
            }
        }

        ans
    }
}
