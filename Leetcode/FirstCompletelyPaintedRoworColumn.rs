// https://leetcode.com/problems/first-completely-painted-row-or-column

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        let m = mat[0].len();

        let mut ij = HashMap::new();
        for i in 0..n {
            for j in 0..m {
                ij.insert(mat[i][j], (i, j));
            }
        }

        let mut rows = vec![0; n];
        let mut cols = vec![0; m];
        assert!(n * m == arr.len());
        for (ind, x) in arr.into_iter().enumerate() {
            let &(i, j) = ij.get(&x).unwrap();
            rows[i] += 1;
            cols[j] += 1;

            if rows[i] == m || cols[j] == n {
                return ind as i32;
            }
        }

        unreachable!()
    }
}
