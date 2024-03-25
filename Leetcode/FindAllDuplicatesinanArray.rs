// https://leetcode.com/problems/find-all-duplicates-in-an-array

struct Solution {}

impl Solution {
    pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for i in 0..nums.len() {
            let ind = nums[i].abs() as usize;
            if nums[ind - 1] < 0 {
                ans.push(ind as i32);
            }

            nums[ind - 1] *= -1;
        }

        ans
    }
}
