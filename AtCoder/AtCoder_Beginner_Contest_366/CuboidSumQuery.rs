use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let ni = n;
        let nj = n;
        let nk = n;

        let mut arr: Vec<Vec<Vec<i64>>> = vec![vec![vec![0; nk]; nj]; ni]; // [ni][nj][nk]
        for i in 0..ni {
            for j in 0..nj {
                for k in 0..nk {
                    arr[i][j][k] = cin.next::<i64>();
                }
            }
        }

        let mut pre = vec![vec![vec![0; nk + 1]; nj + 1]; ni + 1];

        for i in 1..=ni {
            pre[i][0][0] = pre[i - 1][0][0] + arr[i - 1][0][0];
        }
        for j in 1..=nj {
            pre[0][j][0] = pre[0][j - 1][0] + arr[0][j - 1][0]
        }
        for k in 1..=nk {
            pre[0][0][k] = pre[0][0][k - 1] + arr[0][0][k - 1];
        }

        for k in 1..=nk {
            for i in 1..=ni {
                pre[k][i][0] = arr[k - 1][i - 1][0] + pre[k - 1][i][0] + pre[k][i - 1][0]
                    - pre[k - 1][i - 1][0];
            }
        }
        for i in 1..=ni {
            for j in 1..=nj {
                pre[0][i][j] = arr[0][i - 1][j - 1] + pre[0][i - 1][j] + pre[0][i][j - 1]
                    - pre[0][i - 1][j - 1];
            }
        }
        for j in 1..=nj {
            for k in 1..=nk {
                pre[k][0][j] = arr[k - 1][0][j - 1] + pre[k - 1][0][j] + pre[k][0][j - 1]
                    - pre[k - 1][0][j - 1];
            }
        }

        for k in 1..=nk {
            for i in 1..=ni {
                for j in 1..=nj {
                    pre[k][i][j] = arr[k - 1][i - 1][j - 1]
                        + pre[k - 1][i][j]
                        + pre[k][i - 1][j]
                        + pre[k][i][j - 1]
                        - pre[k - 1][i - 1][j]
                        - pre[k][i - 1][j - 1]
                        - pre[k - 1][i][j - 1]
                        + pre[k - 1][i - 1][j - 1];
                }
            }
        }

        let q = cin.next::<usize>();
        for _ in 0..q {
            let (lx, rx, ly, ry, lz, rz) = (
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
            );

            let ans = pre[rx][ry][rz]
                - (pre[lx - 1][ry][rz] + pre[rx][ly - 1][rz] + pre[rx][ry][lz - 1])
                + (pre[lx - 1][ly - 1][rz] + pre[lx - 1][ry][lz - 1] + pre[rx][ly - 1][lz - 1])
                - (pre[lx - 1][ly - 1][lz - 1]);

            println!("{ans}");
        }
    }
}
