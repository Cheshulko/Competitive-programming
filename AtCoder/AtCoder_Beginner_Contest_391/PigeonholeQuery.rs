#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
use std::mem::swap;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &std::path::Path) -> Self {
        use std::fs::File;

        let tokens = VecDeque::new();
        let file = File::open(&path).expect("Expect file exists");
        Self {
            reader: Box::new(BufReader::new(file)),
            tokens,
        }
    }
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self {
            reader: Box::new(BufReader::new(std::io::stdin())),
            tokens,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer).unwrap();
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
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N, Q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut ans = 0;

        let mut Cnt = vec![1; N];
        let mut At = (0..N).collect::<Vec<_>>();

        for _ in 0..Q {
            let t = cin.next::<usize>();
            if t == 1 {
                let (p, h) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);

                let at = At[p];
                At[p] = h;

                Cnt[at] -= 1;
                if Cnt[at] == 1 {
                    ans -= 1;
                }

                Cnt[h] += 1;
                if Cnt[h] == 2 {
                    ans += 1;
                }
            } else {
                println!("{ans}");
            }
        }
    }
}
