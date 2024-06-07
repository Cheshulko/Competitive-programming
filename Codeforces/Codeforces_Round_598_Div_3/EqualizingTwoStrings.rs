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

        let s = cin.next::<String>();
        let t = cin.next::<String>();

        let mut c_cnt_s = vec![0; 26];
        let mut c_cnt_t = vec![0; 26];

        let mut s = s.into_bytes();
        let mut t = t.into_bytes();

        for &c in s.iter() {
            c_cnt_s[(c - b'a') as usize] += 1;
        }
        for &c in t.iter() {
            c_cnt_t[(c - b'a') as usize] += 1;
        }

        if c_cnt_s.iter().zip(c_cnt_t.iter()).any(|(a, b)| a != b) {
            println!("NO");
            continue;
        }

        if c_cnt_s.iter().any(|x| *x > 1) || c_cnt_t.iter().any(|x| *x > 1) {
            println!("YES");
            continue;
        }

        let mut a = 0;
        let mut b = 0;

        for i in 0..(n - 1) {
            for j in 0..(n - i - 1) {
                if s[j] > s[j + 1] {
                    a += 1;
                    s.swap(j, j + 1);
                }
            }
        }

        for i in 0..(n - 1) {
            for j in 0..(n - i - 1) {
                if t[j] > t[j + 1] {
                    b += 1;
                    t.swap(j, j + 1);
                }
            }
        }

        if a % 2 != b % 2 {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
