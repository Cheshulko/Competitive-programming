// https://leetcode.com/problems/student-attendance-record-ii

struct Solution {}

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let m = 1_000_000_000 + 7;
        let n = n as usize;

        let mut dp0 = vec![vec![0; 3]; 2];
        dp0[0][0] = 1_i128;

        for _ in 1..=n {
            let mut dp1 = vec![vec![0; 3]; 2];

            dp1[0][0] += dp0[0][0] + dp0[0][1] + dp0[0][2];
            dp1[0][0] %= m;
            for r in 1..=2 {
                dp1[0][r] += dp0[0][r - 1];
                dp1[0][r] %= m;
            }

            dp1[1][0] += dp0[0][0] + dp0[0][1] + dp0[0][2];
            dp1[1][0] += dp0[1][0] + dp0[1][1] + dp0[1][2];
            dp1[1][0] %= m;
            for r in 1..=2 {
                dp1[1][r] += dp0[1][r - 1];
                dp1[1][r] %= m;
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        let mut ans = 0;
        ans = (ans + dp0[0][0] + dp0[0][1] + dp0[0][2]) % m;
        ans = (ans + dp0[1][0] + dp0[1][1] + dp0[1][2]) % m;

        ans as i32
    }
}
