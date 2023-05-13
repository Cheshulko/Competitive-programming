// https://leetcode.com/contest/biweekly-contest-104/problems/sum-in-a-matrix/

impl Solution {
    pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
        let mut nums = nums;
        nums.iter_mut().for_each(|x| {
            x.sort_by(|a, b| a.cmp(&b).reverse());
        });

        let mut ans = 0;

        let mut cur = vec![0; nums[0].len()];

        for i in 0..nums.len() {
            for j in 0..nums[i].len() {
                cur[j] = cur[j].max(nums[i][j]);
            }
        }
        ans = cur.iter().sum();

        ans
    }
}
