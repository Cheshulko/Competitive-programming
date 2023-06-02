// https://leetcode.com/problems/detonate-the-maximum-bombs

struct Solution {}

impl Solution {
    pub fn maximum_detonation(bombs: Vec<Vec<i32>>) -> i32 {
        let mut used = vec![false; 101];

        let mut ans = 0;
        for (i, bomb) in bombs.iter().enumerate() {
            let mut used = vec![false; 101];
            ans = ans.max(Solution::dfs(i, &mut used, &bombs));
        }
        ans
    }

    fn d2(x1: i32, y1: i32, x2: i32, y2: i32, r: i32) -> bool {
        let dx = (x1 - x2) as i64;
        let dy = (y1 - y2) as i64;
        let r64 = r as i64;
        let r2 = r64 * r64;

        dx * dx + dy * dy <= r2
    }

    fn dfs(i: usize, used: &mut Vec<bool>, bombs: &Vec<Vec<i32>>) -> i32 {
        used[i] = true;
        let bomb_i = &bombs[i];

        let mut cnt = 1;
        for (j, bomb_j) in bombs.iter().enumerate() {
            if !used[j] {
                if Solution::d2(bomb_i[0], bomb_i[1], bomb_j[0], bomb_j[1], bomb_i[2]) {
                    cnt += Solution::dfs(j, used, bombs);
                }
            }
        }
        cnt
    }
}
