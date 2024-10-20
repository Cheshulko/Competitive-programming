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
    let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut b = vec![0; n];
        for i in 0..n {
            b[i] = cin.next::<usize>();
        }

        let ab = a.iter().zip(b.iter()).collect::<Vec<_>>();
        let mut abi = ab
            .into_iter()
            .enumerate()
            .map(|(i, (x, y))| (x, y, i))
            .collect::<Vec<_>>();
        abi.sort_unstable();

        let mut s = 0;
        let mut bh = BinaryHeap::<(usize, usize)>::new();
        for i in 0..k - 1 {
            s += abi[i].1;
            bh.push((*abi[i].1, i));
        }

        let mut ans = usize::MAX;
        for i in (k - 1)..n {
            s += abi[i].1;
            bh.push((*abi[i].1, i));
            ans = ans.min(abi[i].0 * s);

            let l = bh.pop().unwrap();
            s -= l.0;
        }

        println!("{ans}");
    }

    Ok(())
}
