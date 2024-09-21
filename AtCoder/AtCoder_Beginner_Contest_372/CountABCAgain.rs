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
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = cin.next::<String>().into_bytes();

        let mut ans = 0;
        let mut used = vec![false; n];
        for i in 0..n - 2 {
            if s[i] == b'A' && s[i + 1] == b'B' && s[i + 2] == b'C' {
                for j in 0..3 {
                    used[i + j] = true;
                }
                ans += 1;
            }
        }

        for _ in 0..q {
            let (x, c) = (cin.next::<usize>(), cin.next::<char>());
            let x = x - 1;
            let c = c as u8;

            let bef = s[x];
            if bef == c {
            } else {
                if used[x] {
                    if s[x] == b'A' {
                        used[x] = false;
                        used[x + 1] = false;
                        used[x + 2] = false;
                    } else if s[x] == b'B' {
                        used[x] = false;
                        used[x - 1] = false;
                        used[x + 1] = false;
                    } else if s[x] == b'C' {
                        used[x] = false;
                        used[x - 1] = false;
                        used[x - 2] = false;
                    } else {
                        unreachable!();
                    }

                    ans -= 1;
                }

                s[x] = c;

                if !used[x] {
                    if s[x] == b'A' && x + 2 < n && s[x + 1] == b'B' && s[x + 2] == b'C' {
                        used[x] = true;
                        used[x + 1] = true;
                        used[x + 2] = true;
                        ans += 1
                    } else if s[x] == b'B'
                        && x > 0
                        && x + 1 < n
                        && s[x - 1] == b'A'
                        && s[x + 1] == b'C'
                    {
                        used[x] = true;
                        used[x - 1] = true;
                        used[x + 1] = true;
                        ans += 1
                    } else if s[x] == b'C' && x >= 2 && s[x - 2] == b'A' && s[x - 1] == b'B' {
                        used[x] = true;
                        used[x - 1] = true;
                        used[x - 2] = true;
                        ans += 1
                    }
                }
            }

            println!("{ans}");
        }
    }

    Ok(())
}
