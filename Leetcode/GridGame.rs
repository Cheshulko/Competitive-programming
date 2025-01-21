// https://leetcode.com/problems/grid-game

struct Solution {}

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let grid: [Vec<i64>; 2] = [
            grid[0].iter().map(|x| *x as i64).collect(),
            grid[1].iter().map(|x| *x as i64).collect(),
        ];

        let mut up = grid[0].iter().sum::<i64>();
        let mut down = 0;

        let mut ans = i64::MAX;
        for i in 0..grid[0].len() {
            up -= grid[0][i];
            ans = ans.min(up.max(down));
            down += grid[1][i];
        }

        ans
    }
}
