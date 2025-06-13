// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs

struct Solution {}

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();

        nums.sort_unstable();

        let mut l = -1;
        let mut r = *nums.iter().max().unwrap() + 1;

        while r - l > 1 {
            let m = l + ((r - l) >> 1);
            let mut cnt = 0;
            let mut i = 0;

            while i + 1 < n {
                if nums[i + 1] - nums[i] <= m {
                    cnt += 1;

                    i += 2;
                } else {
                    i += 1;
                }
            }

            if cnt >= p {
                r = m;
            } else {
                l = m;
            }
        }

        r
    }
}
