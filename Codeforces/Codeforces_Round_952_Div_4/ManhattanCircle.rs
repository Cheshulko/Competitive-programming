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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![];
        for i in 0..n {
            let s = cin.next::<String>();
            a.push(s.into_bytes());
        }

        let mut u = n - 1;
        let mut d = 0;
        let mut l = m - 1;
        let mut r = 0;
        for i in 0..n {
            let ci = a[i].iter().position(|x| x == &b'#');
            let ei = a[i].iter().rev().position(|x| x == &b'#');

            if let Some(ci) = ci {
                if i < u {
                    u = i;
                }
                if i > d {
                    d = i;
                }
                let ei = m - ei.unwrap() - 1;

                if ci < l {
                    l = ci;
                }
                if ei > r {
                    r = ei;
                }
            }
        }

        let x = (r + l) / 2 + 1;
        let y = (d + u) / 2 + 1;

        println!("{y} {x} ");
    }
}
