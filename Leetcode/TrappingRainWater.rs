// https://leetcode.com/problems/trapping-rain-water

pub struct SegmentTree {
    t: Vec<(i32, i32, bool)>, // (add, summa, reset)
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        SegmentTree {
            t: vec![(0, 0, false); 4 * n],
        }
    }

    pub fn push(&mut self, v: usize, left_pos: usize, middle: usize, right_pos: usize) {
        if self.t[v].2 {
            self.t[v].2 = false;

            self.t[2 * v].0 = 0;
            self.t[2 * v].1 = 0;
            self.t[2 * v].2 = true;
            self.t[2 * v + 1].0 = 0;
            self.t[2 * v + 1].1 = 0;
            self.t[2 * v + 1].2 = true;
        }

        if self.t[v].0 > 0 {
            self.t[2 * v].0 += self.t[v].0;
            self.t[2 * v].1 += (middle + 1 - left_pos) as i32 * self.t[v].0;
            self.t[2 * v + 1].0 += self.t[v].0;
            self.t[2 * v + 1].1 += (right_pos - middle) as i32 * self.t[v].0;
            self.t[v].0 = 0;
        }
    }

    pub fn reset(&mut self, v: usize, left_pos: usize, right_pos: usize, l: usize, r: usize) {
        if l > r {
            return;
        }
        if left_pos == l && right_pos == r {
            self.t[v].0 = 0;
            self.t[v].1 = 0;
            self.t[v].2 = true;
            return;
        }
        let middle = (left_pos + right_pos) / 2;
        self.push(v, left_pos, middle, right_pos);
        self.reset(2 * v, left_pos, middle, l, middle.min(r));
        self.reset(2 * v + 1, middle + 1, right_pos, l.max(middle + 1), r);
    }

    pub fn add(
        &mut self,
        v: usize,
        left_pos: usize,
        right_pos: usize,
        l: usize,
        r: usize,
        value: i32,
    ) {
        if l > r {
            return;
        }
        if left_pos == l && right_pos == r {
            self.t[v].0 += value;
            self.t[v].1 += (r - l + 1) as i32 * value;
            return;
        }
        let middle = (left_pos + right_pos) / 2;
        self.push(v, left_pos, middle, right_pos);
        self.add(2 * v, left_pos, middle, l, middle.min(r), value);
        self.add(
            2 * v + 1,
            middle + 1,
            right_pos,
            l.max(middle + 1),
            r,
            value,
        );
        self.t[v].1 = self.t[2 * v].1 + self.t[2 * v + 1].1;
    }

    pub fn sum(&mut self, v: usize, left_pos: usize, right_pos: usize, l: usize, r: usize) -> i32 {
        if l > r {
            return 0;
        }
        if left_pos == l && right_pos == r {
            return self.t[v].1;
        }
        let middle = (left_pos + right_pos) / 2;
        self.push(v, left_pos, middle, right_pos);
        return self.sum(2 * v, left_pos, middle, l, middle.min(r))
            + self.sum(2 * v + 1, middle + 1, right_pos, l.max(middle + 1), r);
    }
}

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let max = *height.iter().max().unwrap() as usize;
        let mut st = SegmentTree::new(max + 1);

        let mut top = 0;
        let mut ans = 0;

        for h in height.into_iter().rev() {
            let h = h as usize;

            if top >= h {
                let sum = st.sum(1, 0, max, 0, h);
                st.reset(1, 0, max, 0, h);
                st.add(1, 0, max, h + 1, top, 1);
                ans += sum;
            } else {
                let sum = st.sum(1, 0, max, 0, top);
                st.reset(1, 0, max, 0, h);
                ans += sum;
            }
            top = top.max(h);
        }

        ans
    }
}
