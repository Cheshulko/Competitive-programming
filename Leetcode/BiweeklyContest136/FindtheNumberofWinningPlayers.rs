// https://leetcode.com/problems/find-the-number-of-winning-players

struct Solution {}

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let p = pick.len();

        let mut win = vec![false; 100];
        let mut cnt = vec![vec![0; 100]; 100];
        let mut ans = 0;

        for i in 0..p {
            let pl = pick[i][0] as usize;
            let col = pick[i][1] as usize;

            cnt[pl][col] += 1;
            if cnt[pl][col] == pl + 1 {
                win[pl] = true;
            }
        }

        for i in 0..100 {
            if win[i] {
                ans += 1;
            }
        }

        ans
    }
}
