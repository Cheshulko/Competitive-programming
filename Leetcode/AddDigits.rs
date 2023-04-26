// https://leetcode.com/problems/add-digits

struct Solution {}

impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        let mut cnt = 0;
        while num > 0 {
            cnt += num % 10;
            num /= 10;
        }
        if cnt > 9 {
            Solution::add_digits(cnt)
        } else {
            cnt
        }
    }
}
