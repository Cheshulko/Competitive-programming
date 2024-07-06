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

fn to_vec(mut num: usize, n: usize) -> Vec<usize> {
    let mut v = vec![];
    while num > 0 {
        v.push(num % 3);
        num /= 3;
    }
    while v.len() < n + 2 {
        v.push(0);
    }
    v
}

fn to_num(v: Vec<usize>) -> usize {
    let mut p = 1;
    let mut res = 0;
    for x in v.into_iter() {
        res += x * p;
        p *= 3;
    }

    res
}

fn move_pair(pos: usize, num: usize, n: usize) -> usize {
    let mut v = to_vec(num, n);

    let mut move_pos = 0;
    for i in 0..(v.len() - 1) {
        if v[i] == 0 && v[i + 1] == 0 {
            move_pos = i;
            break;
        }
    }

    if v[pos] == 0 || v[pos + 1] == 0 {
        return num;
    }

    v[move_pos] = v[pos];
    v[move_pos + 1] = v[pos + 1];

    v[pos] = 0;
    v[pos + 1] = 0;

    to_num(v)
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>().into_bytes();
        let t = cin.next::<String>().into_bytes();

        let mut ss = 0;
        let mut st = 0;

        // ...(3-1)(3-0)
        // 9, 27, 81, 243, 729, 2187

        let mut p = 9;
        for x in s.into_iter().rev() {
            if x == b'W' {
                ss += 1 * p;
            } else {
                ss += 2 * p;
            }
            p *= 3;
        }

        let mut p = 9;
        for x in t.into_iter().rev() {
            if x == b'W' {
                st += 1 * p;
            } else {
                st += 2 * p;
            }
            p *= 3;
        }

        let mut dp = vec![0; 3_usize.pow(2 + n as u32)];
        dp[ss] = 1;

        let mut q = VecDeque::<usize>::new();
        q.push_back(ss);

        while let Some(cur) = q.pop_front() {
            if cur == st {
                break;
            }

            for pos in 0..=n {
                let move_to = move_pair(pos, cur, n);
                if move_to == cur {
                    continue;
                }

                if dp[move_to] == 0 {
                    dp[move_to] = dp[cur] + 1;
                    q.push_back(move_to);
                }
            }
        }

        println!("{ans}", ans = dp[st] - 1);
    }
}
