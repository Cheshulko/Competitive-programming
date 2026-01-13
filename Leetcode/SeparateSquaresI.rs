// https://leetcode.com/problems/separate-squares-i

struct Solution;

impl Solution {
    const EPS: f64 = 1e-5;

    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        fn find(squares: &[Vec<i32>], y: f64) -> (f64, f64) {
            let mut d = 0.;
            let mut u = 0.;

            for sq in squares.iter() {
                let l = sq[2] as f64;
                let y1 = sq[1] as f64;
                let y2 = y1 + l;

                match ((y1 < y), (y2 < y)) {
                    (true, true) => d += l * l,
                    (true, false) => {
                        d += (y - y1) * l;
                        u += (y2 - y) * l
                    }
                    (false, false) => u += l * l,
                    _ => unreachable!(),
                }
            }

            (d, u)
        }

        let mut l = 0.;
        let mut r = 1e10;

        while r - l > Solution::EPS {
            let m = (r + l) / 2.;

            let (d, u) = find(&squares, m);
            if u - d > Solution::EPS {
                l = m;
            } else {
                r = m;
            }
        }

        l
    }
}
