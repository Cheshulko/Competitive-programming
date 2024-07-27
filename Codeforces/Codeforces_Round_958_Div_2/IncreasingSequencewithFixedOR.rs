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
    // let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<u64>();

        let mut ans = vec![n];

        for b in 0..64 {
            if n & (1 << b) > 0 {
                let x = n ^ (1 << b);
                if x > 0 {
                    ans.push(x);
                }
            }
        }

        for x in ans.windows(2) {
            assert!(x[0] | x[1] == n);
        }

        println!("{x}", x = ans.len());
        for x in ans.into_iter().rev() {
            print!("{x} ");
        }

        println!();
    }
}
