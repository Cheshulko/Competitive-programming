use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let a = (0..n).map(|_| cin.next::<usize>()).collect::<Vec<_>>();
        let s = cin.next::<String>().into_bytes();

        let mut pref = vec![0; n + 1];
        let mut suf = vec![0; n + 1];
        let mut ans = 0;

        for i in 0..n {
            pref[i + 1] = pref[i] + a[i];
        }
        for i in (0..n).rev() {
            let dx = ((s[i] == b'1') as usize) * a[i];
            suf[i] = suf[i + 1] + dx;
            ans += dx;
        }

        for i in 0..n {
            if s[i] == b'1' {
                ans = ans.max(pref[i] + suf[i + 1]);
            }
        }

        println!("{ans}");
    }
}
