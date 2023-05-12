// https://leetcode.com/problems/solving-questions-with-brainpower

struct Solution {}

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0; n];
        let mut ans = 0;

        for (ind, question) in questions.iter().enumerate() {
            let points = question[0] as i64;
            let skip = question[1] as usize;
            let next = ind + skip + 1;
            if ind > 0 {
                dp[ind] = dp[ind].max(dp[ind - 1]);
            }
            if next < n {
                dp[next] = dp[next].max(dp[ind] + points);
            }
            ans = ans.max(dp[ind] + points);
        }

        ans
    }
}
