// https://leetcode.com/problems/increment-submatrices-by-one

struct Solution {}

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        let mut ans = vec![vec![0; n]; n];
        for q in queries.into_iter() {
            let &[tr, tc, br, bc] = q.as_slice() else {
                panic!();
            };
            let [tr, tc, br, bc] = [tr as usize, tc as usize, br as usize, bc as usize];

            for i in tr..=br {
                ans[i][tc] += 1;
            }
            if bc + 1 < n {
                for i in tr..=br {
                    ans[i][bc + 1] -= 1;
                }
            }
        }

        for i in 0..n {
            for j in 1..n {
                ans[i][j] += ans[i][j - 1];
            }
        }

        ans
    }
}
