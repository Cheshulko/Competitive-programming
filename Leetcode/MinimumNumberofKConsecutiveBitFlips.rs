// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips

struct Solution {}

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, mut k: i32) -> i32 {
        let n = nums.len() as i32;
        k = k.min(n + 1);

        let mut arr = vec![];
        let mut ans = 0;
        for i in 0..(n + 1 - k) {
            let cnt = (arr.len() - arr.partition_point(|x| *x < i - k + 1)) as i32;
            if nums[i as usize] ^ (cnt % 2) == 0 {
                ans += 1;
                arr.push(i);
            }
        }

        for i in (n + 1 - k)..n {
            let cnt = (arr.len() - arr.partition_point(|x| *x < i - k + 1)) as i32;
            if nums[i as usize] ^ (cnt % 2) == 0 {
                return -1;
            }
        }

        return ans;
    }
}
