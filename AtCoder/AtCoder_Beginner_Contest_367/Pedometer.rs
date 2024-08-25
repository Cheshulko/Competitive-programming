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

    let _t = 1;
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        let mut pref = vec![0; n + 1];
        for i in 0..n {
            a[i] = cin.next::<usize>();
            pref[i + 1] = pref[i] + a[i];
        }

        let mut hm = HashMap::<usize, usize>::new();
        for i in 1..n {
            let rem = pref[i] % m;
            *hm.entry(rem).or_default() += 1;
        }

        let mut ans = 0;
        let mut ext = 0;
        for i in 1..=n {
            ans += *hm.get(&ext).unwrap_or(&0);

            let rem_rem = pref[i] % m;
            if let Some(x) = hm.get_mut(&rem_rem) {
                *x -= 1;
            }
            ext = rem_rem;

            let rem_new = (pref[n] + pref[i - 1]) % m;
            *hm.entry(rem_new).or_default() += 1;
        }

        println!("{ans}");
    }
}
