// https://leetcode.com/problems/set-mismatch

struct Solution {}

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let cnt: Vec<usize> = nums.iter().fold(vec![0; n + 1], |mut v, x| {
            v[*x as usize] += 1;
            v
        });

        let n = n as i32;
        let sum = nums.iter().sum::<i32>();

        let x = cnt.iter().position(|x| x == &2).unwrap() as i32;
        let y = (1 + n) * n / 2 - sum + x;

        vec![x, y]
    }
}
