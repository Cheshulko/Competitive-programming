use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

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
        let n = cin.next::<usize>();

        let mut a = vec![];
        for _ in 0..n {
            let x = cin.next::<usize>();
            a.push(x);
        }

        let l = a.iter().position(|x| x == &1).unwrap();
        let r = n - a.iter().rev().position(|x| x == &1).unwrap() - 1;
        let ans = &a[l..=r].iter().filter(|x| x == &&0).count();

        println!("{ans}");
    }
}
