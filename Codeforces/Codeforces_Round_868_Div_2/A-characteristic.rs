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
    'out: for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        for i in 0..=n {
            let j = n - i;

            let mut cnt = 0;
            if i > 1 {
                cnt += i * (i - 1) / 2;
            }

            if j > 1 {
                cnt += j * (j - 1) / 2;
            }

            if cnt == k {
                println!("YES");
                for _ in 0..i {
                    print!("1 ");
                }
                for _ in 0..j {
                    print!("-1 ");
                }
                println!();
                continue 'out;
            }
        }

        println!("NO");
    }
}
