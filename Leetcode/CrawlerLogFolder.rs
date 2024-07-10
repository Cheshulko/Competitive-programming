// https://leetcode.com/problems/crawler-log-folder

struct Solution {}

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut cnt = 0;

        for log in logs.into_iter() {
            match log.as_str() {
                "../" => {
                    cnt -= 1;
                    cnt = cnt.max(0);
                }
                "./" => {}
                _ => cnt += 1,
            }
        }

        cnt
    }
}
