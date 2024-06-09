use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();
    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut max = usize::MIN;
        let mut min = usize::MAX;
        let mut max_ind = 0;
        let mut min_ind = 0;
        for i in 0..n {
            let x = cin.next::<usize>();

            if x > max {
                max = x;
                max_ind = i;
            }

            if x < min {
                min = x;
                min_ind = i;
            }
        }

        let mut ans = usize::MAX;
        ans = ans.min(max_ind.min(min_ind) + 1 + n - max_ind.max(min_ind));
        ans = ans.min(max_ind.max(min_ind) + 1);
        ans = ans.min(n - max_ind.min(min_ind));

        println!("{ans}");
    }
}
