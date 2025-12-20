// https://leetcode.com/problems/delete-columns-to-make-sorted

struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strs = strs.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();
        let n = strs.len();
        let m = strs[0].len();

        let mut ans = 0;
        for j in 0..m {
            let mut ok = true;
            for i in 1..n {
                ok = ok && (strs[i - 1][j] <= strs[i][j]);
            }

            if !ok {
                ans += 1;
            }
        }

        ans
    }
}
