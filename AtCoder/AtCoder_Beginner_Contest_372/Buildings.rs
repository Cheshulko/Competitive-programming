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

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut h = vec![0; n];
        for i in 0..n {
            h[i] = cin.next::<usize>();
        }

        let mut ans = vec![0];
        let mut s = BTreeSet::<(usize, usize)>::new();
        s.insert((h[n - 1], n - 1));

        for i in (0..n - 1).rev() {
            let g = s.len();
            while let Some(ll) = s.first() {
                if ll.0 < h[i] {
                    s.pop_first();
                } else {
                    break;
                }
            }
            s.insert((h[i], i));
            ans.push(g);
        }

        ans.reverse();
        for x in ans.into_iter() {
            print!("{x} ");
        }
        println!();
    }

    Ok(())
}
