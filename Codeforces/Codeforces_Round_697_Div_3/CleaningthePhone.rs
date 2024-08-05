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
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut mem = vec![vec![]; 2];

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        for i in 0..n {
            let b = cin.next::<usize>();
            mem[b - 1].push(a[i]);
        }

        mem[0].sort_unstable();
        mem[1].sort_unstable();

        mem[0].reverse();
        mem[1].reverse();

        let mut ans = usize::MAX;
        let mut pref = vec![vec![0]; 2];
        for i in 0..mem[0].len() {
            let add = pref[0][i] + mem[0][i];
            pref[0].push(add);
            if add >= m {
                ans = ans.min((i + 1) * 1);
            }
        }
        for i in 0..mem[1].len() {
            let add = pref[1][i] + mem[1][i];
            pref[1].push(add);
            if add >= m {
                ans = ans.min((i + 1) * 2);
            }
        }

        for i in 0..=mem[0].len() {
            let f = pref[0][i];

            let need = m - f;
            if need <= 0 {
                break;
            }

            let s = pref[1].partition_point(|x| x < &need);

            if s != pref[1].len() {
                ans = ans.min(i * 1 + s * 2);
            }
        }

        if ans != usize::MAX {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }
}
