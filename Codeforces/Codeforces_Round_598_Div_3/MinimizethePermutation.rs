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
        let mut pos = vec![0; n + 1];
        let mut can = vec![vec![true; n]; n];

        for i in 0..n {
            let x = cin.next::<usize>();
            a.push(x);
            pos[x] = i;
        }

        for i in 1..=n {
            if pos[i] > i - 1 {
                for to in (i..=pos[i]).rev() {
                    if can[to][to - 1] && a[to - 1] > a[to] {
                        can[to][to - 1] = false;
                        pos[i] = to - 1;
                        pos[a[to - 1]] = to;

                        a.swap(to, to - 1);
                    }
                }
            }
        }

        for b in a.into_iter() {
            print!("{b} ");
        }
        println!();
    }
}
