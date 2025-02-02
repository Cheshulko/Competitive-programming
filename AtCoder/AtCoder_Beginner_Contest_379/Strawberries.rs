use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::i64;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &Path) -> Self {
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());
        let s = cin.next::<String>();

        let s = s
            .into_bytes()
            .into_iter()
            .map(|x| x != b'X')
            .collect::<Vec<_>>();

        let mut ans = 0;
        let mut i = 0;
        let mut cnt = 0;
        while i < n {
            if !s[i] {
                i += 1;
            }
            while i < n && s[i] && cnt < k {
                i += 1;
                cnt += 1;
            }
            if cnt == k {
                ans += 1;
                cnt = 0;
            } else {
                cnt = 0;
            }
        }
        println!("{ans}");
    }

    Ok(())
}
