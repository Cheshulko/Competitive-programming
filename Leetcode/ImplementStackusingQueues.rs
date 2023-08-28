use std::collections::VecDeque;

struct MyStack {
    qs: [VecDeque<i32>; 2],
    ind: usize,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            qs: [VecDeque::<i32>::new(), VecDeque::<i32>::new()],
            ind: 0,
        }
    }

    fn push(&mut self, x: i32) {
        self.qs[self.ind].push_back(x);
    }

    fn pop(&mut self) -> i32 {
        let mut ret = i32::MAX;

        while let Some(x) = self.qs[self.ind].pop_front() {
            ret = x;
            if !self.qs[self.ind].is_empty() {
                self.qs[self.ind ^ 1].push_back(x);
            }
        }

        self.ind ^= 1;

        ret
    }

    fn top(&mut self) -> i32 {
        let mut ret = i32::MAX;

        while let Some(x) = self.qs[self.ind].pop_front() {
            ret = x;
            self.qs[self.ind ^ 1].push_back(x);
        }

        self.ind ^= 1;

        ret
    }

    fn empty(&self) -> bool {
        self.qs[self.ind].is_empty() && self.qs[self.ind ^ 1].is_empty()
    }
}
