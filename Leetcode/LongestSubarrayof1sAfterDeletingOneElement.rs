// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element

struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut v = vec![];
        let mut cnt = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                cnt += 1;
                ans = ans.max(cnt);
            } else {
                v.push(cnt);
                cnt = 0;
            }
        }
        v.push(cnt);
        for i in 1..v.len() {
            ans = ans.max(v[i] + v[i - 1]);
        }
        if v.len() == 1 {
            ans -= 1;
        }

        ans
    }
}
