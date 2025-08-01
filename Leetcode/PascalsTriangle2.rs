// https://leetcode.com/problems/pascals-triangle

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![1]];
        for i in 0..num_rows as usize - 1 {
            let mut next = vec![1];
            for j in 0..ans[i].len() - 1 {
                next.push(ans[i][j] + ans[i][j + 1]);
            }

            next.push(1);
            ans.push(next);
        }

        ans
    }
}
