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
        let n = cin.next::<usize>();
        let q = cin.next::<usize>();
        let mut ans = 0;
        let mut cur_l = 0;
        let mut cur_r = 1;

        for i in 0..q {
            let h = cin.next::<char>();
            let t = cin.next::<usize>() - 1;

            if h == 'L' {
                let mut a = usize::MAX;
                {
                    let mut a0 = 0;
                    let mut cur = cur_l;
                    while cur != t {
                        a0 += 1;
                        cur = (cur + 1) % n;

                        if cur == cur_r {
                            a0 = usize::MAX;
                            break;
                        }
                    }
                    a = a.min(a0);
                }
                {
                    let mut a0 = 0;
                    let mut cur = cur_l;
                    while cur != t {
                        a0 += 1;
                        cur = (cur + n - 1) % n;

                        if cur == cur_r {
                            a0 = usize::MAX;
                            break;
                        }
                    }

                    a = a.min(a0);
                }

                cur_l = t;
                ans += a;
            } else {
                let mut a = usize::MAX;
                {
                    let mut a0 = 0;
                    let mut cur = cur_r;
                    while cur != t {
                        a0 += 1;
                        cur = (cur + 1) % n;

                        if cur == cur_l {
                            a0 = usize::MAX;
                            break;
                        }
                    }
                    a = a.min(a0);
                }
                {
                    let mut a0 = 0;
                    let mut cur = cur_r;
                    while cur != t {
                        a0 += 1;
                        cur = (cur + n - 1) % n;

                        if cur == cur_l {
                            a0 = usize::MAX;
                            break;
                        }
                    }

                    a = a.min(a0);
                }
                cur_r = t;
                ans += a;
            }
        }

        println!("{ans}");
    }

    Ok(())
}
