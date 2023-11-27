// https://leetcode.com/problems/knight-dialer

struct Solution {}

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let n = n as usize;
        let mut can = vec![vec![false; 10]; 10];

        can[0][4] = true;
        can[0][6] = true;

        can[1][6] = true;
        can[1][8] = true;

        can[2][7] = true;
        can[2][9] = true;

        can[3][4] = true;
        can[3][8] = true;

        can[4][0] = true;
        can[4][3] = true;
        can[4][9] = true;

        can[6][0] = true;
        can[6][1] = true;
        can[6][7] = true;

        can[7][2] = true;
        can[7][6] = true;

        can[8][1] = true;
        can[8][3] = true;

        can[9][2] = true;
        can[9][4] = true;

        let mut cnt = vec![vec![0; n + 1]; 10];
        for i in 0..10 {
            cnt[i][1] = 1;
        }
        for l in 2..=n {
            for from in 0..10 {
                for to in 0..10 {
                    if can[from][to] {
                        cnt[to][l] = (cnt[to][l] + cnt[from][l - 1]) % (1_000_000_000 + 7);
                    }
                }
            }
        }

        cnt.into_iter()
            .map(|v| v[n])
            .into_iter()
            .reduce(|a, b| (a + b) % (1_000_000_000 + 7))
            .unwrap()
    }
}
