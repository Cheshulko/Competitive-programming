use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;
use std::vec;

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

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, d, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut vs = vec![0; n + 1];
        let mut ve = vec![0; n + 1];

        for i in 0..k {
            let (l, r) = (cin.next::<usize>(), cin.next::<usize>());
            let l = l - 1;
            let r = r - 1;

            vs[l] += 1;
            ve[r + 1] += 1;
        }

        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut cur = 0;
        for i in 0..(d - 1) {
            cur += vs[i];
            cur -= ve[i];
        }

        let mut d0 = 0;
        let mut d1 = 0;
        for i in (d - 1)..n {
            cur += vs[i];
            cur -= ve[i - (d - 1)];
            if cur < min {
                min = cur;
                d0 = i - (d - 1);
            }
            if cur > max {
                max = cur;
                d1 = i - (d - 1);
            }
        }

        let d1 = d1 + 1;
        let d0 = d0 + 1;

        println!("{d1} {d0}");
    }

    Ok(())
}
