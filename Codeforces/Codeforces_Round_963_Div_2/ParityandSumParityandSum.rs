use core::num;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::rc;
use std::slice::*;
use std::usize;
use std::vec;

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

        let mut a = vec![0; n];
        let mut bs = BTreeSet::<(i128, usize)>::new();
        let mut cnt = 0;
        for i in 0..n {
            a[i] = cin.next::<i128>();

            if a[i] % 2 == 0 {
                cnt += 1;
                bs.insert((a[i], i));
            }
        }

        if cnt == n || cnt == 0 {
            println!("0");
            continue;
        }

        a.sort_unstable();
        let mut max_odd = 0;
        for i in (0..n).rev() {
            if a[i] % 2 == 1 {
                max_odd = a[i];
                break;
            }
        }

        let mut ans = 0;
        while !bs.is_empty() {
            let gg = if let Some((p, i)) = bs.range(..(max_odd, usize::MAX)).next_back() {
                (*p, *i)
            } else {
                (-1, 0)
            };

            if gg.0 == -1 {
                ans = cnt + 1;
                break;
            } else {
                max_odd += gg.0;
                bs.remove(&gg);
                ans += 1;
            }
        }

        println!("{ans}");
    }
}
