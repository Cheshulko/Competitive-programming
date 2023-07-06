// https://leetcode.com/problems/minimum-size-subarray-sum

struct Solution {}

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut ans = usize::MAX;
        let mut sum = nums[i];
        while i < nums.len() {
            if sum >= target {
                ans = ans.min(j - i + 1);

                if j - i > 0 {
                    sum -= nums[i];
                    i += 1;
                } else {
                    i += 1;
                    j += 1;
                    sum = *nums.get(i).unwrap_or(&i32::MAX);
                }
            } else {
                if j + 1 < nums.len() {
                    j += 1;
                    sum += nums[j];
                    if sum >= target {
                        ans = ans.min(j - i + 1);
                        sum -= nums[i];
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
        }

        if ans == usize::MAX {
            ans = 0;
        }

        ans as i32
    }
}
