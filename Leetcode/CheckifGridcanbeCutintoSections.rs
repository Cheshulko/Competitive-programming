// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections

struct Solution {}

impl Solution {
    pub fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        // Horizontal
        {
            rectangles.sort_by(|a, b| a[0].cmp(&b[0]));

            let mut cnt = 0;
            let mut end = rectangles[0][2];
            for rectangle in rectangles.iter() {
                cnt += (rectangle[0] >= end) as usize;
                end = end.max(rectangle[2]);
            }

            if cnt >= 2 {
                return true;
            }
        }

        // Vertical
        {
            rectangles.sort_by(|a, b| a[1].cmp(&b[1]));

            let mut cnt = 0;
            let mut end = rectangles[0][3];
            for rectangle in rectangles.iter() {
                cnt += (rectangle[1] >= end) as usize;
                end = end.max(rectangle[3]);
            }

            if cnt >= 2 {
                return true;
            }
        }

        false
    }
}
