// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle

struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut ma = 0;

        for dimension in dimensions {
            let (h, w) = (dimension[0], dimension[1]);
            let d2 = h * h + w * w;

            if d2 > ma {
                ans = w * h;
                ma = d2;
            } else if d2 == ma {
                ans = ans.max(w * h);
            }
        }

        ans
    }
}
