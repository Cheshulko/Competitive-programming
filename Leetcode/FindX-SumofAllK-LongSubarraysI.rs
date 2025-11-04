// https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-i

struct Solution {}

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        use std::collections::BTreeSet;

        let k = k as usize;
        let x = x as usize;
        let mut cnt = vec![0; 55];
        let mut freq = BTreeSet::<(usize, usize)>::new();

        for i in 0..k - 1 {
            let n = nums[i] as usize;
            if let Some((x, _)) = freq.take(&(cnt[n], n)) {
                freq.insert((x + 1, n));
            } else {
                freq.insert((1, n));
            }

            cnt[n] += 1;
        }

        let mut ans = vec![];
        for i in (k - 1)..nums.len() {
            let n = nums[i] as usize;
            if let Some((x, _)) = freq.take(&(cnt[n], n)) {
                freq.insert((x + 1, n));
            } else {
                freq.insert((1, n));
            }

            cnt[n] += 1;
            let mut sum = 0;
            for &(x, y) in freq.iter().rev().take(x) {
                sum += x * y;
            }

            ans.push(sum as i32);

            let n = nums[i - (k - 1)] as usize;
            if let Some((x, _)) = freq.take(&(cnt[n], n)) {
                freq.insert((x - 1, n));
            }

            cnt[n] -= 1;
        }

        ans
    }
}
