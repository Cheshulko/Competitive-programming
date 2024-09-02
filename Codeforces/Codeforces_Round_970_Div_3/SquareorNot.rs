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
        let mut n = cin.next::<usize>();
        let mut s = cin.next::<String>().into_bytes();

        let mut i = 1;
        while i * i <= n {
            if i * i > n {
                break;
            }
            if i * i == n {
                break;
            }
            i += 1;
        }

        if i * i != n {
            println!("No");
            continue;
        }

        let mut arr = vec![vec![0; i]; i];
        let mut k = 0;
        let mut cnt = 0;
        for j in 0..s.len() {
            k = j / i;
            arr[k][j % i] = (s[j] == b'1') as usize;
            cnt += (s[j] == b'1') as usize;
        }

        let mut have = 0;
        let mut can = true;
        for j in 0..i {
            can &= (arr[0][j] == 1);
            have += (arr[0][j] == 1) as usize;
        }
        for j in 0..i {
            can &= (arr[i - 1][j] == 1);
            have += (arr[i - 1][j] == 1) as usize;
        }
        for j in 0..i {
            can &= (arr[j][0] == 1);
            have += (arr[j][0] == 1) as usize;
        }
        for j in 0..i {
            can &= (arr[j][i - 1] == 1);
            have += (arr[j][i - 1] == 1) as usize;
        }

        if can {
            if have > 4 {
                if have - 4 == cnt {
                    println!("Yes");
                } else {
                    println!("No");
                }
            } else {
                println!("No");
            }
        } else {
            println!("No");
        }
    }
}
