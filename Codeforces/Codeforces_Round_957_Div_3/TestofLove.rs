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
    // let _t = 1;
    for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let s_ = cin.next::<String>().into_bytes();
        let mut s = vec![b'L'];
        s.extend(s_.into_iter());
        s.push(b'W'); // n + 2

        let mut dp = vec![0; 21 + n + 2];
        dp[0] = k + 1;

        let mut i = 0;
        while i <= (n + 1) {
            if dp[i] != 0 {
                if s[i] == b'C' {
                } else if s[i] == b'W' {
                    dp[i + 1] = dp[i + 1].max(dp[i] - 1);
                } else if s[i] == b'L' {
                    for x in 1..=m {
                        dp[i + x] = dp[i + x].max(dp[i]);
                    }
                }
            }

            i += 1;
        }

        if dp[n + 1] != 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
