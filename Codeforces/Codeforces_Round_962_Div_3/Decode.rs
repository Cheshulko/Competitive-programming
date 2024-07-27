use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
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

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        // let (n, x) = (cin.next::<usize>(), cin.next::<usize>());

        let s = cin.next::<String>().into_bytes();

        let n = s.len();

        let mut pref = vec![0; n + 1];
        let mut hm = HashMap::<i64, usize>::new();

        for i in 0..n {
            pref[i + 1] = pref[i] + if s[i] == b'1' { 1 } else { -1 };

            *hm.entry(pref[i + 1]).or_default() += n - i;
        }

        const MOD: usize = 1_000_000_000 + 7;
        let mut ans = 0;

        for i in 0..n {
            let cur = pref[i];

            let gg = *hm.entry(cur).or_default();
            let x = (i + 1) * gg;
            ans += x;
            ans %= MOD;

            *hm.entry(pref[i + 1]).or_default() -= n - i;
        }
        println!("{ans}");
    }
}
