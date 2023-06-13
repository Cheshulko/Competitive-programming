// https://leetcode.com/problems/equal-row-and-column-pairs

struct Solution {}

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        grid.iter().fold(0, |res, row| {
            res + grid.iter().enumerate().fold(0, |res_for_row, (ind, _)| {
                let column = grid.iter().map(|row| row[ind]).collect::<Vec<i32>>();
                res_for_row + (row == &column) as i32
            })
        })
    }
}
