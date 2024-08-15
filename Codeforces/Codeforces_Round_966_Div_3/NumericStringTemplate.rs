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

        for i in 0..n {
            arr[i] = cin.next::<i64>();
        }

        let m = cin.next::<usize>();
        for _ in 0..m {
            let mut ok = true;

            let mut hm = HashMap::<i64, u8>::new();
            let s = cin.next::<String>().into_bytes();

            if s.len() != n {
                println!("NO");
                continue;
            }

            let mut sym = HashMap::<u8, i64>::new();

            for (i, x) in s.into_iter().enumerate() {
                if let Some(&ch) = hm.get(&arr[i]) {
                    if ch != x {
                        ok = false;
                    }
                } else {
                    if let Some(v) = sym.get(&x) {
                        ok = false;
                        break;
                    } else {
                        sym.insert(x, arr[i]);
                        hm.insert(arr[i], x);
                    }
                }
            }

            if ok {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
