// https://leetcode.com/problems/find-the-largest-area-of-square-inside-two-rectangles

struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let n = bottom_left.len();

        let mut ans = 0;
        for i in 0..n {
            for j in 0..i {
                let lx = bottom_left[i][0].max(bottom_left[j][0]);
                let ly = bottom_left[i][1].max(bottom_left[j][1]);

                let rx = top_right[i][0].min(top_right[j][0]);
                let ry = top_right[i][1].min(top_right[j][1]);

                let l = (rx - lx).min(ry - ly);
                if l > 0 {
                    ans = ans.max(l as i64 * l as i64);
                }
            }
        }

        ans
    }
}
