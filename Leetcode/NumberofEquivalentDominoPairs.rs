// https://leetcode.com/problems/number-of-equivalent-domino-pairs

struct Solution {}

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut cnt = [[0; 9]; 9];

        for domino in dominoes {
            let x = domino[0].min(domino[1]) as usize - 1;
            let y = domino[0].max(domino[1]) as usize - 1;

            cnt[x][y] += 1;
        }

        let mut ans = 0;
        for i in 0..9 {
            for j in i..9 {
                let x = cnt[i][j];
                if x > 0 {
                    ans += (x - 1) * x / 2
                }
            }
        }

        ans
    }
}
