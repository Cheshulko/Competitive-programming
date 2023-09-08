// https://leetcode.com/problems/pascals-triangle

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut ans: Vec<Vec<i32>> = vec![];
        ans.push(vec![1]);

        for i in 1..num_rows {
            let prev = &ans[i - 1];
            let mut cur = vec![1];
            for j in 0..prev.len() {
                let v = prev[j] + prev.get(j + 1).unwrap_or(&0);
                cur.push(v);
            }
            ans.push(cur);
        }

        ans
    }
}
