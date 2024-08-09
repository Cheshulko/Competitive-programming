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

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();

        let mut cnt = vec![0; 30000];
        for i in 0..n - 1 {
            cnt[(s[i] - b'A') as usize * 100 + (s[i + 1] - b'A') as usize] += 1;
        }

        let mut ind = 0;
        let mut mx = 0;
        for i in 0..30000 {
            if cnt[i] > mx {
                mx = cnt[i];
                ind = i;
            }
        }

        let a = ((ind / 100) as u8 + b'A') as char;
        let b = ((ind % 100) as u8 + b'A') as char;
        println!("{a}{b}");
    }
}
