// https://leetcode.com/problems/divide-array-into-equal-pairs

struct Solution {}

impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut cnt = vec![0; 500 + 1];
        for num in nums {
            cnt[num as usize] += 1;
        }

        cnt.iter().all(|&x| x & 1 == 0)
    }
}
