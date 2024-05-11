use std::cmp::*;
use std::collections::*;

impl Solution {
    pub fn max_points_inside_square(mut points: Vec<Vec<i32>>, s: String) -> i32 {
        let s = s.into_bytes();

        let mut have = vec![false; 1000];

        let mut points = points
            .into_iter()
            .enumerate()
            .map(|(i, a)| (a, i))
            .collect::<Vec<_>>();
        points.sort_by_key(|(x, _)| x[0].abs().max(x[1].abs()));

        let mut ans = 0;
        for (i, (p, j)) in points.iter().enumerate() {
            let j = *j;
            if have[s[j] as usize] {
                let x = p[0];
                let y = p[1];

                let z = x.abs().max(y.abs());

                for k in 0..i {
                    if points[k].0[0].abs() == z.abs() || points[k].0[1].abs() == z.abs() {
                        ans -= 1;
                    }
                }
                break;
            } else {
                have[s[j] as usize] = true;
                ans += 1;
            }
        }

        ans
    }
}
