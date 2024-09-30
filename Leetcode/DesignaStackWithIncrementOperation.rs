// https://leetcode.com/problems/design-a-stack-with-increment-operation

struct CustomStack {
    max_size: usize,
    v: Vec<(i32, i32)>,
    cur_size: usize,
}

impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            max_size: maxSize as usize,
            v: vec![(0, 0); maxSize as usize],
            cur_size: 0,
        }
    }

    fn push(&mut self, x: i32) {
        if self.cur_size != self.max_size {
            self.v[self.cur_size as usize] = (x, 0);
            self.cur_size += 1;
        }
    }

    fn pop(&mut self) -> i32 {
        if self.cur_size == 0 {
            return -1;
        } else {
            let (x, dx) = self.v[self.cur_size - 1];
            self.cur_size -= 1;
            if self.cur_size != 0 {
                self.v[self.cur_size - 1].1 += dx;
            }

            return x + dx;
        }
    }

    fn increment(&mut self, k: i32, val: i32) {
        let k = (k as usize).min(self.cur_size);
        if k != 0 {
            self.v[k as usize - 1].1 += val;
        }
    }
}
