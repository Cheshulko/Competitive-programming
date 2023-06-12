// https://leetcode.com/problems/summary-ranges

struct Solution {}

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            vec![]
        } else {
            let mut st = nums[0];
            let mut ans = vec![];

            for i in 1..nums.len() {
                if nums[i] - nums[i - 1] != 1 {
                    if st == nums[i - 1] {
                        ans.push(st.to_string());
                    } else {
                        ans.push(format!("{}->{}", st, nums[i - 1]));
                    }

                    st = nums[i];
                }
            }

            if st == nums[nums.len() - 1] {
                ans.push(st.to_string());
            } else {
                ans.push(format!("{}->{}", st, nums[nums.len() - 1]));
            }

            ans
        }
    }
}
