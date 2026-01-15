// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid

struct Solution;

impl Solution {
    pub fn maximize_square_hole_area(
        n: i32,
        m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        h_bars.sort_unstable();
        v_bars.sort_unstable();

        let mut h = 1;
        if !h_bars.is_empty() {
            let mut hc = 1;
            let mut hb = h_bars[0] - 1;
            for i in 0..h_bars.len() {
                if hb + 1 == h_bars[i] && h_bars[i] != 1 && h_bars[i] != n + 2 {
                    hc += 1;
                } else {
                    if h_bars[i] != 1 && h_bars[i] != n + 2 {
                        hc = 2;
                    } else {
                        hc = 1;
                    }
                }
                hb = h_bars[i];
                h = h.max(hc);
            }
        }

        let mut v = 1;
        if !v_bars.is_empty() {
            let mut vc = 1;
            let mut vb = v_bars[0] - 1;
            for i in 0..v_bars.len() {
                if vb + 1 == v_bars[i] && v_bars[i] != 1 && v_bars[i] != m + 2 {
                    vc += 1;
                } else {
                    if v_bars[i] != 1 && v_bars[i] != m + 2 {
                        vc = 2;
                    } else {
                        vc = 1;
                    }
                }
                vb = v_bars[i];
                v = v.max(vc);
            }
        }

        let m = h.min(v);
        m * m
    }
}
