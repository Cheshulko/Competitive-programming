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

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, x) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut can = vec![false; n];
        can[x - 1] = true;
        for _ in 0..m {
            let r = cin.next::<usize>();
            let c = cin.next::<char>();

            let mut can2 = vec![false; n];
            can = match c {
                '0' => {
                    for x in 0..n {
                        if can[x] {
                            can2[(x + r) % n] = true;
                        }
                    }
                    can2
                }
                '1' => {
                    for x in 0..n {
                        if can[x] {
                            can2[(n + x - r) % n] = true;
                        }
                    }
                    can2
                }
                '?' => {
                    for x in 0..n {
                        if can[x] {
                            can2[(x + r) % n] = true;
                            can2[(n + x - r) % n] = true;
                        }
                    }
                    can2
                }
                _ => unreachable!(),
            };
        }

        let x = can
            .into_iter()
            .enumerate()
            .filter_map(|(i, x)| x.then_some(i + 1))
            .collect::<Vec<_>>();
        println!("{l}", l = x.len());
        for y in x.into_iter() {
            print!("{y} ");
        }
        println!();
    }
}
