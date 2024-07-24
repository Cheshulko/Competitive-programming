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
    // let _t = 1;
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        if k == 0 {
            println!("0")
        } else if k <= n {
            println!("1");
        } else {
            let mut cnt = 1;
            let mut k = k - n;
            for i in (1..=(n - 1)).rev() {
                if k <= i {
                    cnt += 1;
                    println!("{cnt}");
                    break;
                } else {
                    cnt += 1;
                    k -= i;
                }
                if k <= i {
                    cnt += 1;
                    println!("{cnt}");
                    break;
                } else {
                    cnt += 1;
                    k -= i;
                }
            }
        }
    }
}
