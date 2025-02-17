use core::num;
use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
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
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut adj = vec![vec![]; n];
        for i in 0..m {
            let (a, b) = (cin.next::<usize>(), cin.next::<usize>());
            let (a, b) = (a - 1, b - 1);

            adj[a].push(b);
        }

        let mut dist = vec![usize::MAX; n];
        let mut q = VecDeque::<(usize, usize)>::new();
        dist[0] = 0;
        q.push_back((0, 0));
        let mut used = vec![false; n];
        used[0] = true;

        let mut can = false;
        let mut dd = 0;

        while let Some((d, v)) = q.pop_front() {
            for &to in adj[v].iter() {
                if !used[to] {
                    used[to] = true;
                    q.push_back((d + 1, to));
                } else {
                    if to == 0 && !can {
                        can = true;
                        dd = d + 1;
                    }
                }
            }
        }

        if can {
            println!("{dd}");
        } else {
            println!("-1");
        }
    }

    Ok(())
}
