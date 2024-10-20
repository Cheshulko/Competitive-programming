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

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut b = vec![0; n - 1];
        for i in 0..n - 1 {
            b[i] = cin.next::<usize>();
        }

        a.sort_unstable();
        b.sort_unstable();

        let mut ans = usize::MAX;
        let mut can_buy = true;

        let mut ai = n as i32 - 1;
        let mut bi = n as i32 - 2;

        while ai >= 0 && bi >= 0 {
            if a[ai as usize] <= b[bi as usize] {
                ai -= 1;
                bi -= 1;
            } else {
                if can_buy {
                    can_buy = false;
                    ans = a[ai as usize];
                    ai -= 1;
                } else {
                    println!("-1");
                    continue 'test;
                }
            }
        }

        if can_buy {
            println!("{}", a[0]);
        } else {
            println!("{ans}");
        }
    }

    Ok(())
}
