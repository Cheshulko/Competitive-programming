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
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut x = vec![0; n];
        for i in 0..n {
            x[i] = cin.next::<i64>();
        }
        let mut cnt = HashMap::<i64, i64>::new();
        let mut prev = x[0] + 1;
        let mut open = n as i64 - 2;
        let mut cur = n as i64 - 1;
        let mut close = 1;
        *cnt.entry(cur).or_default() += 1;

        for i in 1..n {
            let dx = x[i] - 1 - prev + 1;
            if dx > 0 {
                *cnt.entry(cur).or_default() += dx;
            }
            cur += open;
            open -= 1;
            *cnt.entry(cur).or_default() += 1;
            cur -= close;
            close += 1;
            prev = x[i] + 1;
        }

        for _ in 0..q {
            let k = cin.next::<i64>();
            let ans = cnt.get(&k).unwrap_or(&0);
            print!("{ans} ");
        }
        println!();
    }

    Ok(())
}
