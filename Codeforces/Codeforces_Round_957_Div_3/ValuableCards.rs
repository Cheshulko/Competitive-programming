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
        let (n, x) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = HashSet::<usize>::new();
        s.insert(x);

        let mut ans = 0;

        for _ in 0..n {
            let y = cin.next::<usize>();

            let mut s2 = HashSet::<usize>::new();
            for &k in s.iter() {
                if k % y != 0 {
                    continue;
                }

                if k == y {
                    ans += 1;

                    s2.clear();
                    s.clear();
                    s.insert(x);
                    s.insert(x / k);

                    break;
                }

                s2.insert(k / y);
            }

            s.extend(s2);
        }

        ans += 1;
        println!("{ans}");
    }
}
