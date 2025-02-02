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
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();
        let mut s = s
            .into_iter()
            .map(|x| (x - b'0') as usize)
            .collect::<Vec<_>>();

        let mut ans = vec![];
        let mut cnt = 0;

        for i in 0..n {
            let x = (i + 1) * s[i];
            cnt += x;
            ans.push(cnt);
        }

        ans.reverse();

        let mut add = 0;
        for i in 0..n {
            ans[i] += add;

            add = ans[i] / 10;
            ans[i] %= 10;
        }

        while add > 0 {
            ans.push(add % 10);
            add /= 10;
        }

        ans.reverse();
        for x in ans.into_iter() {
            print!("{x}");
        }
        println!();
    }

    Ok(())
}
