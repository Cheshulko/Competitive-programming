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
        let q = cin.next::<usize>();

        let mut dq = VecDeque::<(usize, usize)>::new();
        let mut tt = 0;

        for _ in 0..q {
            let t = cin.next::<usize>();

            match t {
                1 => {
                    dq.push_back((tt, 0));
                }
                2 => {
                    let t = cin.next::<usize>();
                    tt += t;
                }
                3 => {
                    let h = cin.next::<usize>();
                    let mut ans = 0;
                    while let Some((ht, hh)) = dq.front() {
                        let dt = tt - ht;
                        if hh + dt >= h {
                            ans += 1;
                            dq.pop_front();
                        } else {
                            break;
                        }
                    }
                    println!("{ans}");
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(())
}
