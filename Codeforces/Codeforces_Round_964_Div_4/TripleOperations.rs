use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::rc;
use std::slice::*;
use std::usize;
use std::vec;

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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (l, r) = (cin.next::<usize>(), cin.next::<usize>());

        let mut min = l;

        let mut cnt = 0;
        while min > 0 {
            min /= 3;
            cnt += 1;
        }

        let mut pow = 1;
        let mut cur = l + 1;
        for i in 0..cnt {
            pow *= 3;
        }

        let mut ans = 2 * cnt;

        while cur <= r {
            let next = (pow - 1).min(r);
            if next >= cur {
                ans += cnt * (next + 1 - cur);
            }
            cur = next + 1;
            cnt += 1;
            pow *= 3;
        }

        println!("{ans}");
    }
}
