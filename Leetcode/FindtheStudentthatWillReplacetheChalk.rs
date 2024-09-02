// https://leetcode.com/problems/find-the-student-that-will-replace-the-chalk

struct Solution {}

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let k = k as i64;
        let chalk = chalk.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let s = chalk.iter().sum::<i64>();

        let cnt = k / s;
        let k = k - s * cnt;

        let mut ss = 0;
        for i in 0..chalk.len() {
            ss += chalk[i];
            if ss > k {
                return i as i32;
            }
        }

        unreachable!()
    }
}
