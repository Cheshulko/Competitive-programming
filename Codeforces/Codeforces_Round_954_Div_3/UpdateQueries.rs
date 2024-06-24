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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = cin.next::<String>().into_bytes();

        let mut ind = vec![0; m];
        let mut cnt = vec![0; n + 1];
        for i in 0..m {
            ind[i] = cin.next::<usize>();
            cnt[ind[i] - 1] += 1;
        }

        let mut c = cin.next::<String>().into_bytes();
        c.sort_unstable();

        let mut st = 0;
        for i in 0..n {
            if cnt[i] == 0 {
                continue;
            }

            s[i] = c[st];
            st += 1;
            cnt[i] -= 1;
            while cnt[i] > 0 {
                cnt[i] -= 1;
            }
        }

        let ans = s.into_iter().map(|x| x as char).collect::<String>();
        println!("{ans}");
    }
}
