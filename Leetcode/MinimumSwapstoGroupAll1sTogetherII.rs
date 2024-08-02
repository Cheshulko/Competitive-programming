// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together-ii

struct Solution {}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let ones = nums.iter().filter(|&x| *x == 1).count();
        if ones == n || ones == 0 {
            return 0;
        }

        let mut ans = usize::MAX;

        let mut s = 0;
        let mut i = 0;
        while i < ones - 1 {
            s += nums[i] as usize;
            i += 1;
        }

        for j in 0..n {
            s += nums[(i + j) % n] as usize;
            ans = ans.min(ones - s);
            s -= nums[(n + 1 + i + j - ones) % n] as usize;
        }

        ans as i32
    }
}
