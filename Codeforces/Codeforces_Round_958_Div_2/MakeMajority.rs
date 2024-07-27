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
        let n = cin.next::<usize>();
        let mut a = vec![0; n];

        let s = cin.next::<String>().into_bytes();

        for i in 0..n {
            a[i] = (s[i] == b'1') as i32;
        }

        let mut i = 0;
        let mut z = 0;
        let mut o = 0;
        while i < n {
            if a[i] == 0 {
                z += 1;
                while i < n && a[i] == 0 {
                    i += 1;
                }
            } else {
                o += 1;
                i += 1;
            }
        }

        if o > z {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
