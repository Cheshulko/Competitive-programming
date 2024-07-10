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

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut ans = 0;
        let mut cm = 0;
        let mut cz = 0;
        for _ in 0..n {
            let x = cin.next::<i64>();
            if x < 0 {
                cm += 1;
                ans += -1 - x;
            } else if x > 0 {
                ans += x - 1;
            } else {
                cz += 1;
                ans += 1;
            }
        }

        if cm % 2 == 1 && cz == 0 {
            ans += 2;
        }

        println!("{ans}");
    }
}
