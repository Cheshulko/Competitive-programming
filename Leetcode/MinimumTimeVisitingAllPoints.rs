// https://leetcode.com/problems/minimum-time-visiting-all-points

struct Solution {}

impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|pair| {
                let x1 = pair[0][0];
                let x2 = pair[1][0];
                let y1 = pair[0][1];
                let y2 = pair[1][1];

                let dx = (x1 - x2).abs();
                let dy = (y1 - y2).abs();

                dx + dy - dx.min(dy)
            })
            .sum()
    }
}
