use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let n = cin.next::<usize>();

        if n % 2 == 0 {
            println!("-1");
            continue;
        }

        let mut ans = vec![0; n];
        let mut l = 0;
        let mut r = n - 1;
        let mut cur = n;
        ans[n / 2] = 1;
        while r > l {
            ans[l] = cur;
            l += 1;
            cur -= 1;
            ans[r] = cur;
            r -= 1;
            cur -= 1;
        }

        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
