use core::num;
use std::cmp::*;
use std::collections::*;
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

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = VecDeque::<usize>::new();
        let mut b = VecDeque::<usize>::new();

        for i in 0..n {
            let x = cin.next::<usize>();
            a.push_back(x);
        }

        for i in 0..n {
            let x = cin.next::<usize>();
            b.push_back(x);
        }

        let mut have_a = vec![true; n + 1];
        let mut have_b = vec![true; n + 1];
        let mut alice = false;

        for i in 0..n - 1 {
            if a.front() != b.front() && a.front() != b.back() {
                alice = true;
                break;
            } else {
                if a.back() != b.front() && a.back() != b.back() {
                    alice = true;
                    break;
                } else {
                    have_a[*a.front().unwrap()] = false;
                    a.pop_front();
                }
            }

            if !have_a[*b.front().unwrap()] {
                have_b[*b.front().unwrap()] = false;
                b.pop_front();
            } else if !have_a[*b.back().unwrap()] {
                have_b[*b.back().unwrap()] = false;
                b.pop_back();
            } else {
                have_b[*b.back().unwrap()] = false;
                b.pop_back();
            }
        }

        if alice {
            println!("Alice");
        } else {
            println!("Bob");
        }
    }
}
