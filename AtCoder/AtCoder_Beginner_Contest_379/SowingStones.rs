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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut x = vec![0; m];
        for i in 0..m {
            x[i] = cin.next::<usize>() - 1;
        }

        let mut a = vec![0; m];
        for i in 0..m {
            a[i] = cin.next::<usize>();
        }

        let mut xa = x.into_iter().zip(a.into_iter()).collect::<Vec<_>>();
        xa.push((n, usize::MAX));
        xa.sort_unstable();

        if xa[0].0 != 0 {
            println!("-1");
            continue 'test;
        }

        let mut can = true;
        let mut ans = 0;

        for i in 0..m {
            let j = i + 1;
            let pj = xa[j].0;
            let dp = pj - xa[i].0 - 1;
            if xa[i].1 <= dp {
                can = false;
                break;
            }
            let free = xa[i].1 - dp - 1;
            ans += (1 + dp) * dp / 2;
            ans += free * (dp + 1);
            xa[j].1 += free;

            if pj == n && free != 0 {
                can = false;
            }
        }

        if !can {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }

    Ok(())
}
