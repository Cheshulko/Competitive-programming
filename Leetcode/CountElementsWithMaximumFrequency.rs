// https://leetcode.com/problems/count-elements-with-maximum-frequency

struct Solution {}

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];

        for num in nums.into_iter() {
            cnt[num as usize] += 1;
        }

        let max = *cnt.iter().max().unwrap();

        max * cnt.into_iter().filter(|x| x == &max).count() as i32
    }
}
