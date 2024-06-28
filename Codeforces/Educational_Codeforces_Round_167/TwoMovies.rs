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

        let mut a = vec![0; n];
        let mut b = vec![0; n];

        for i in 0..n {
            a[i] = cin.next::<i32>();
        }
        for i in 0..n {
            b[i] = cin.next::<i32>();
        }

        let mut s1 = 0;
        let mut s2 = 0;
        let mut d1 = 0;
        let mut d2 = 0;

        for i in 0..n {
            if a[i] > b[i] {
                s1 += a[i];
            } else if a[i] < b[i] {
                s2 += b[i];
            } else if a[i] == 1 {
                d1 += 1;
            } else if a[i] == -1 {
                d2 -= 1;
            }
        }

        while d1 > 0 {
            if s1 < s2 {
                s1 += 1;
            } else {
                s2 += 1;
            }
            d1 -= 1;
        }

        while d2 < 0 {
            if s1 < s2 {
                s2 -= 1;
            } else {
                s1 -= 1;
            }
            d2 += 1;
        }

        println!("{ans}", ans = s1.min(s2));
    }
}
