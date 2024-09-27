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

fn ask(l: i64, r: i64, cin: &mut Cin) -> i64 {
    println!("? {l} {r}", l = l + 1, r = r + 1);
    let _ = stdout().flush();

    cin.next::<i64>()
}

// [l; r]
fn search(cur: usize, l: i64, r: i64, k: i64, arr: &mut Vec<i64>, cin: &mut Cin) {
    if r <= l {
        println!("! {ans}", ans = l + 1);
        let _ = stdout().flush();

        return;
    }

    let m = (l + r) >> 1;
    // [l; m]
    if arr[2 * cur + 1] == -1 {
        arr[2 * cur + 1] = ask(l, m, cin);
    }
    let z = m - l + 1 - arr[2 * cur + 1];
    if z >= k {
        // go left
        search(2 * cur + 1, l, m, k, arr, cin);
        arr[2 * cur + 1] += 1;
    } else {
        // go right
        search(2 * cur + 2, m + 1, r, k - z, arr, cin);
        arr[2 * cur + 2] += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let (n, t) = (cin.next::<i64>(), cin.next::<usize>());
    let mut pow_2 = n.ilog2() as usize;
    if n & (n - 1) != 0 {
        pow_2 += 1;
    }
    let pow_2_len = 1 << pow_2;
    let mut arr = vec![-1; 2 * pow_2_len];

    for _ in 0..t {
        let k = cin.next::<i64>();

        let l = 0;
        let r = n - 1;
        search(0, l, r, k, &mut arr, &mut cin);
    }

    Ok(())
}
