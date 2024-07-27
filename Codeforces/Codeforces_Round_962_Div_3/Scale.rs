use core::num;
use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = vec![];
        for i in 0..n {
            let x = cin.next::<String>().into_bytes();
            s.push(x);
        }

        let m = n / k;

        let mut ans = vec![vec![0; m]; m];

        for i in 0..m {
            for j in 0..m {
                let mut sum = 0;
                for i_ in (i * k)..(i * k + k) {
                    for j_ in (j * k)..(j * k + k) {
                        sum = (s[i_][j_] == b'1') as usize;
                    }
                }

                ans[i][j] = sum;
            }
        }

        for i in 0..m {
            for j in 0..m {
                print!("{x}", x = ans[i][j]);
            }
            println!();
        }
    }
}
