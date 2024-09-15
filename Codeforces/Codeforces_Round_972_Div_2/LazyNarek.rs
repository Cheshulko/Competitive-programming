use std::cmp;
use std::cmp::*;
use std::collections::*;
use std::i32;
use std::i64;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::ops::Add;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = vec![];
        for i in 0..n {
            let ss = cin.next::<String>().into_bytes();
            s.push(ss);
        }
        let lat = [b'n', b'a', b'r', b'e', b'k'];
        let mut can = vec![false; 1000];
        for x in lat.iter() {
            can[*x as usize] = true;
        }

        let mut dp = vec![vec![i64::MIN; lat.len()]; n + 1];
        dp[0][0] = 0;

        let mut ans = 0;

        for i in 0..n {
            for st in 0..lat.len() {
                dp[i + 1][st] = dp[i][st];
            }
            for st in 0..lat.len() {
                let mut snar: i64 = 0;
                let mut sgp: i64 = 0;
                let mut cur_n = st;

                for j in 0..s[i].len() {
                    if lat[cur_n] == s[i][j] {
                        cur_n += 1;
                    } else {
                        if can[s[i][j] as usize] {
                            sgp += 1;
                        }
                    }
                    if cur_n == lat.len() {
                        cur_n = 0;
                        snar += 5;
                    }
                }

                let d = snar - sgp;

                dp[i + 1][cur_n] = dp[i + 1][cur_n].max(dp[i][cur_n]);
                if dp[i][st] != i64::MIN {
                    dp[i + 1][cur_n] = dp[i + 1][cur_n].max(dp[i][st] + d);
                }
            }
        }

        for st in 0..lat.len() {
            if dp[n][st] != i64::MIN {
                ans = ans.max(dp[n][st] - st as i64);
            }
        }

        println!("{ans}");
    }
}
