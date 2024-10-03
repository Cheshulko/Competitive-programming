// https://leetcode.com/problems/make-sum-divisible-by-p

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        nums.push(0);
        let p = p as usize;
        let n = nums.len();

        let mut rems = HashMap::<usize, usize>::new();
        rems.insert(0, 0);

        let mut used = 0;
        let mut rest = nums.iter().sum::<usize>();
        let mut ans = 0;

        for i in 0..n {
            let rem = rest % p;
            let need = (p - rem) % p;
            if let Some(&pos) = rems.get(&need) {
                ans = ans.max(n - i + pos);
            }

            used += nums[i];
            rest -= nums[i];
            rems.insert(used % p, i + 1);
        }

        let ans = n - ans;
        if ans == n - 1 {
            -1
        } else {
            ans as i32
        }
    }
}
