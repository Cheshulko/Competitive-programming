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

        let mut cnt = HashMap::<usize, usize>::new();
        for _ in 0..n {
            let mut x = cin.next::<usize>();

            let mut i = 2;
            while i * i <= x {
                while x % i == 0 {
                    *cnt.entry(i).or_default() += 1;
                    x /= i;
                }
                i += 1;
            }
            if x > 1 {
                *cnt.entry(x).or_default() += 1;
            }
        }

        let mut ans = 0;
        let mut ones = 0;
        for &v in cnt.values() {
            ans += v / 2;
            ones += v % 2;
        }
        ans += ones / 3;

        println!("{ans}");
    }
}
