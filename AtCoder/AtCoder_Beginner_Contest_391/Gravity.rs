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
        let (N, W) = (cin.next::<usize>(), cin.next::<usize>());

        let mut XY = vec![];
        let mut Cols = vec![vec![]; W];
        for i in 0..N {
            let (x, y) = (cin.next::<usize>() - 1, cin.next::<usize>() - 1);
            XY.push((x, y, i));

            Cols[x].push((y, i));
        }

        let mut mi_at_col = usize::MAX;
        for i in 0..W {
            Cols[i].sort_unstable();

            mi_at_col = mi_at_col.min(Cols[i].len());
        }

        let mut Ans = vec![usize::MAX; N];
        let mut time = 0;

        for r in 0..mi_at_col {
            let mut top = 0;

            for c in 0..W {
                top = top.max(Cols[c][r].0);
            }

            time = top;

            for c in 0..W {
                Ans[Cols[c][r].1] = time;
            }
        }

        let Q = cin.next::<usize>();
        for _ in 0..Q {
            let (t, a) = (cin.next::<usize>(), cin.next::<usize>() - 1);

            if Ans[a] >= t {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
