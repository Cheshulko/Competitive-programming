// https://leetcode.com/problems/implement-queue-using-stacks

struct MyQueue {
    v: [Vec<i32>; 2],
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            v: [vec![], vec![]],
        }
    }

    fn push(&mut self, x: i32) {
        self.v[1].push(x);
    }

    fn pop(&mut self) -> i32 {
        self.re();
        self.v[0].pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.re();
        *self.v[0].last().unwrap()
    }

    fn re(&mut self) {
        if self.v[0].is_empty() {
            self.v[0].extend(self.v[1].clone().iter().rev());
            self.v[1].clear();
        }
    }

    fn empty(&self) -> bool {
        self.v.iter().all(Vec::<i32>::is_empty)
    }
}
