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

    let _t = 1;
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());
        if k > n {
            println!("-1");
            continue;
        }

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }
        a.sort_unstable();

        if k == n {
            println!("{x}", x = a[n - 1]);
            continue;
        }

        if k == 0 {
            if n > 1 && a[0] == a[1] && a[0] == 1 {
                println!("-1");
            } else {
                if a[0] > 1 {
                    println!("1");
                } else {
                    println!("-1");
                }
            }
        } else {
            if a[k - 1] == a[k] {
                println!("-1");
            } else {
                println!("{x}", x = a[k - 1]);
            }
        }
    }
}
