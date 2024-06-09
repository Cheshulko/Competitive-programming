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

fn calc(mut n: usize) -> usize {
    let mut i = 2;
    let mut cnt = 0;
    while i * i <= n {
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
        cnt += 1;
    }

    cnt
}

fn main() {
    let mut cin = Cin::new();
    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let (a, b, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        if k == 1 {
            if a == b {
                println!("No");
            } else if !((a % b == 0) || b % a == 0) {
                println!("No");
            } else {
                println!("Yes");
            }
            continue;
        }

        let ar_count = calc(a);
        let br_count = calc(b);

        if ar_count + br_count >= k {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
