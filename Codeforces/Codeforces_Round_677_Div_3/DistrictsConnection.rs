use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;

struct Cin {
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self { tokens }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap();
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();

    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut a = vec![];
        let mut hs = HashMap::<usize, usize>::new();
        for i in 0..n {
            let x = cin.next::<usize>();
            a.push(x);

            hs.insert(x, i);
        }

        if hs.len() == 1 {
            println!("NO");
            continue;
        }

        let mut hsi = hs.into_iter();
        let (fv, fi) = hsi.next().unwrap();
        let (_, si) = hsi.next().unwrap();

        println!("YES");
        for i in 0..n {
            if a[i] == fv {
                if i == fi {
                    continue;
                }
                println!("{x} {y}", x = i + 1, y = si + 1);
            } else {
                println!("{x} {y}", x = i + 1, y = fi + 1);
            }
        }
    }
}
