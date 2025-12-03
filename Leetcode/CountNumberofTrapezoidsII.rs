// https://leetcode.com/problems/count-number-of-trapezoids-ii

struct Solution {}

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        fn gcd(a: i64, b: i64) -> i64 {
            let mut a = a.abs();
            let mut b = b.abs();
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        use std::collections::HashMap;

        let n = points.len();

        let mut lines = HashMap::<(i64, i64, i64), i32>::new();
        let mut vectors = HashMap::<(i64, i64), HashMap<(i64, i64, i64), i32>>::new();

        for i in 0..n {
            for j in (i + 1)..n {
                let x1 = points[i][0] as i64;
                let y1 = points[i][1] as i64;
                let x2 = points[j][0] as i64;
                let y2 = points[j][1] as i64;

                let mut A = y1 - y2;
                let mut B = x2 - x1;
                let mut C = x1 * y2 - x2 * y1;

                // Normalize sign
                if A < 0 || (A == 0 && B < 0) {
                    A *= -1;
                    B *= -1;
                    C *= -1;
                }

                let common = gcd(A, gcd(B, C));
                let line_key = (A / common, B / common, C / common);

                *lines.entry(line_key).or_default() += 1;

                let mut dx = x1 - x2;
                let mut dy = y1 - y2;
                if dx < 0 || (dx == 0 && dy < 0) {
                    dx = -dx;
                    dy = -dy;
                }
                *vectors
                    .entry((dx, dy))
                    .or_default()
                    .entry(line_key)
                    .or_default() += 1;
            }
        }

        let mut lines_by_k = HashMap::<(i64, i64), Vec<i32>>::new();
        for ((A, B, _), count) in lines.into_iter() {
            lines_by_k.entry((A, B)).or_default().push(count);
        }

        let mut answer = 0;
        for (_, counts) in lines_by_k.into_iter() {
            answer += counts
                .into_iter()
                .fold((0, 0), |(mut pref, mut res), cur| {
                    res += pref * cur;
                    pref += cur;
                    (pref, res)
                })
                .1;
        }

        let mut double_count = 0;
        for (_, line_map) in vectors {
            let counts: Vec<i32> = line_map.values().cloned().collect();
            let mut vec_sum = 0;
            let mut prefix = 0;
            for count in counts {
                vec_sum += prefix * count;
                prefix += count;
            }
            double_count += vec_sum;
        }

        answer -= double_count / 2;

        answer as i32
    }
}
