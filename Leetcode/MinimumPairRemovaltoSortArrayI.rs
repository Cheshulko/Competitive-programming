// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i

struct Solution;

impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        loop {
            let mut st = 0;
            let mut s = i32::MAX;
            let mut ok = true;

            for i in 0..nums.len() - 1 {
                ok = ok && (nums[i] <= nums[i + 1]);
                if nums[i] + nums[i + 1] < s {
                    s = nums[i] + nums[i + 1];
                    st = i;
                }
            }

            if ok {
                return ans;
            }

            nums[st] = s;
            nums.remove(st + 1);
            ans += 1;
        }
    }
}
