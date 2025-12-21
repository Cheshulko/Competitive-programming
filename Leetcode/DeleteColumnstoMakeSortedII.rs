// https://leetcode.com/problems/delete-columns-to-make-sorted-ii

struct Solution {}

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        use std::cmp::Ordering;

        let strs = strs.into_iter().map(|s| s.into_bytes()).collect::<Vec<_>>();

        let n = strs.len();
        let m = strs[0].len();

        let mut ans = 0;
        let mut ok = vec![false; n - 1];
        for j in 0..m {
            let mut rem = false;
            for i in 0..n - 1 {
                match strs[i][j].cmp(&strs[i + 1][j]) {
                    Ordering::Greater if !ok[i] => {
                        rem = true;
                        ans += 1;
                        break;
                    }
                    _ => {}
                }
            }

            if !rem {
                for i in 0..n - 1 {
                    match strs[i][j].cmp(&strs[i + 1][j]) {
                        Ordering::Less => ok[i] = true,
                        _ => {}
                    }
                }
            }
        }

        ans
    }
}
