// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row

struct Solution {}

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        let halves = [tops, bottoms];

        let mut ans = i32::MAX;
        for half1 in 0..2 {
            let half2 = 1 - half1;

            for v in 1..=6 {
                let mut can = true;
                let mut cur = 0;

                for i in 0..n {
                    if halves[half1][i] == v {
                        // Ok
                    } else if halves[half2][i] == v {
                        cur += 1;
                    } else {
                        can = false;
                        break;
                    }
                }

                if can {
                    ans = ans.min(cur)
                }
            }
        }

        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
