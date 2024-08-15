use core::num;
use std::cmp::*;
use std::collections::*;
use std::fmt::Display;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Rem;
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

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut arr = vec![0; n];

        let mut pref = vec![0; n + 1];
        for i in 0..n {
            arr[i] = cin.next::<i64>();
            pref[i + 1] = pref[i] + arr[i];
        }

        let s = cin.next::<String>().into_bytes();

        let mut ll = vec![];
        let mut rr = vec![];
        for i in 0..n {
            if s[i] == b'L' {
                ll.push(i);
            }
        }

        for i in (0..n).rev() {
            if s[i] == b'R' {
                rr.push(i);
            }
        }

        let mut ss = 0;
        let len = ll.len().min(rr.len());
        for i in 0..len {
            let l = ll[i];
            let r = rr[i];
            if l > r {
                break;
            }

            ss += pref[r + 1] - pref[l];
        }
        println!("{ss}");
    }
}
