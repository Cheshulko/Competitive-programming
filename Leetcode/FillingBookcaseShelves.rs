// https://leetcode.com/problems/filling-bookcase-shelves

struct Solution {}

impl Solution {
    fn solve(books: &[Vec<i32>], dp: &mut Vec<i32>, shelf_width: i32) -> i32 {
        let n = books.len();
        if dp[n] != i32::MAX {
            return dp[n];
        }

        let mut width = 0;
        let mut height = 0;
        for (i, x) in books.iter().enumerate() {
            if width + x[0] <= shelf_width {
                height = height.max(x[1]);
                width += x[0];

                dp[n] = dp[n].min(height + Solution::solve(&books[(i + 1)..], dp, shelf_width));
            } else {
                break;
            }
        }

        dp[n]
    }

    pub fn min_height_shelves(mut books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        Solution::solve(&books, &mut dp, shelf_width)
    }
}
