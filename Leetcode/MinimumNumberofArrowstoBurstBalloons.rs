// https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons

struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        points.sort_unstable();

        let mut ans = 0;
        let mut pos = -1;
        let mut i = 0;

        while i < n {
            let point = &points[i];

            let mut left = point[0];
            let mut right = point[1];

            while i < n && right >= points[i][0] {
                left = left.max(points[i][0]);
                right = right.min(points[i][1]);
                i += 1;
            }

            ans += 1;
            pos = right;
        }

        ans
    }
}
