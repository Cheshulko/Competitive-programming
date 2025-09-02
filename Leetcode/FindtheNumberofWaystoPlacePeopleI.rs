// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i

struct Solution {}

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();

        let mut cnt = 0;
        for i in 0..n {
            'out: for j in 0..n {
                if i == j {
                    continue;
                }
                if points[i][0] > points[j][0] {
                    continue;
                }
                if points[i][1] < points[j][1] {
                    continue;
                }

                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }
                    if points[i][0] <= points[k][0]
                        && points[k][0] <= points[j][0]
                        && points[i][1] >= points[k][1]
                        && points[k][1] >= points[j][1]
                    {
                        continue 'out;
                    }
                }

                cnt += 1;
            }
        }

        cnt
    }
}
