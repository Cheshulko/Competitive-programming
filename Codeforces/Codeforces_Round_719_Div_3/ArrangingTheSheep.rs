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
        let n = cin.next::<usize>();

        let o = cin.next::<String>().into_bytes();
        let mut pref = vec![usize::MAX; n + 1];
        let mut suf = vec![usize::MAX; n + 1];

        pref[0] = 0;
        let mut cnt = 0;
        for i in 0..n {
            if o[i] == b'*' {
                pref[i + 1] = pref[i];
                cnt += 1;
            } else {
                pref[i + 1] = pref[i] + cnt;
            }
        }

        suf[n] = 0;
        let mut cnt = 0;
        for i in (0..n).rev() {
            if o[i] == b'*' {
                suf[i] = suf[i + 1];
                cnt += 1;
            } else {
                suf[i] = suf[i + 1] + cnt;
            }
        }

        let mut ans = usize::MAX;
        let mut i = 0;
        while i < n {
            if o[i] == b'*' {
                let mut j = i;
                while j < n && o[j] == b'*' {
                    j += 1;
                }
                ans = ans.min(pref[i] + suf[j]);
                i = j;
            } else {
                i += 1;
            }
        }

        if ans == usize::MAX {
            println!("0");
        } else {
            println!("{ans}");
        }
    }

    Ok(())
}
