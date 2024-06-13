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
        let (x, y, z, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut ans = 0;

        for i in 1..=x {
            for j in 1..=y {
                let ij = i * j;
                if ij > k {
                    continue;
                }
                if k % ij != 0 {
                    continue;
                }
                let p = k / ij;
                if p > z {
                    continue;
                }

                let tt = (x + 1 - i) * (y + 1 - j) * (z + 1 - p);
                ans = ans.max(tt);
            }
        }

        println!("{ans}");
    }
}
