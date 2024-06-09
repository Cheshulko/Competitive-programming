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

fn calc(mut x: i128) -> i128 {
    let mut d = vec![];
    while x > 0 {
        d.push(x % 10);
        x /= 10;
    }

    let mut cur = 0;
    let mut ans = 0;

    for x in d.into_iter().rev() {
        cur = cur * 10 + x;
        ans += cur;
    }

    ans
}

fn main() {
    let mut cin = Cin::new();
    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (l, r) = (cin.next::<i128>(), cin.next::<i128>());

        let ans = calc(r) - calc(l);
        println!("{ans}");
    }
}
