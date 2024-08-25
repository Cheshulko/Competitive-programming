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

fn go(i: usize, st: &mut Vec<usize>, s: usize, r: &Vec<usize>, k: usize) {
    if i == r.len() - 1 {
        for y in 1..=r[i] {
            if (s + y) % k == 0 {
                for &x in st.iter() {
                    print!("{x} ");
                }
                println!("{y}");
            }
        }
    } else {
        for y in 1..=r[i] {
            st.push(y);
            go(i + 1, st, s + y, r, k);
            st.pop();
        }
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut r = vec![0; n];
        for i in 0..n {
            r[i] = cin.next::<usize>();
        }

        let mut st = vec![];
        go(0, &mut st, 0, &r, k);
    }
}
