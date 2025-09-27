// https://leetcode.com/problems/largest-triangle-area

struct Solution {}

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();

        let mut ans: f64 = 0.;
        for i in 0..n {
            let (x1, y1) = (points[i][0] as f64, points[i][1] as f64);
            for j in (i + 1)..n {
                let (x2, y2) = (points[j][0] as f64, points[j][1] as f64);
                for k in (j + 1)..n {
                    let (x3, y3) = (points[k][0] as f64, points[k][1] as f64);

                    ans = ans.max(0.5 * ((x1 - x3) * (y2 - y1) - (x1 - x2) * (y3 - y1)).abs());
                }
            }
        }

        ans
    }
}
