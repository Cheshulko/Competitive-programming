#![allow(non_snake_case)]
#![allow(unused_imports)]

use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, BufReader, BufWriter};
use std::mem::swap;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &std::path::Path) -> Self {
        use std::fs::File;

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

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (N,) = (cin.next::<usize>(),);

        let S = cin
            .next::<String>()
            .chars()
            .into_iter()
            .map(|c| (c == '1') as usize)
            .collect::<Vec<_>>();

        let mut Levels = vec![];
        Levels.push(S.clone());

        loop {
            let last = Levels.last().unwrap();
            if last.len() == 1 {
                break;
            }

            let mut next = vec![];
            for i in (0..last.len()).step_by(3) {
                let b = last[i] + last[i + 1] + last[i + 2];
                next.push((b >= 2) as usize);
            }

            Levels.push(next);
        }

        Levels.reverse();

        fn solve(
            cur_level: usize,
            change_at_ind: usize,
            Levels: &Vec<Vec<usize>>,
            dp: &mut Vec<Vec<usize>>,
        ) -> usize {
            if dp[cur_level][change_at_ind] != usize::MAX {
                return dp[cur_level][change_at_ind];
            }

            let ans = if cur_level == Levels.len() - 1 {
                let s = Levels[cur_level][change_at_ind]
                    + Levels[cur_level][change_at_ind + 1]
                    + Levels[cur_level][change_at_ind + 2];

                if s == 1 || s == 2 {
                    1
                } else {
                    2
                }
            } else {
                let s = Levels[cur_level][change_at_ind]
                    + Levels[cur_level][change_at_ind + 1]
                    + Levels[cur_level][change_at_ind + 2];

                let cur = (s >= 2) as usize;

                let mut mi = usize::MAX;
                match (cur, s) {
                    (0, 0) => {
                        for i in 0..3 {
                            for j in i + 1..3 {
                                mi = mi.min(
                                    solve(cur_level + 1, 3 * (change_at_ind + i), Levels, dp)
                                        + solve(cur_level + 1, 3 * (change_at_ind + j), Levels, dp),
                                );
                            }
                        }
                    }
                    (0, 1) => {
                        for i in 0..3 {
                            if Levels[cur_level][change_at_ind + i] == 0 {
                                mi = mi.min(solve(
                                    cur_level + 1,
                                    3 * (change_at_ind + i),
                                    Levels,
                                    dp,
                                ));
                            }
                        }
                    }
                    (1, 3) => {
                        for i in 0..3 {
                            for j in i + 1..3 {
                                mi = mi.min(
                                    solve(cur_level + 1, 3 * (change_at_ind + i), Levels, dp)
                                        + solve(cur_level + 1, 3 * (change_at_ind + j), Levels, dp),
                                );
                            }
                        }
                    }
                    (1, 2) => {
                        for i in 0..3 {
                            if Levels[cur_level][change_at_ind + i] == 1 {
                                mi = mi.min(solve(
                                    cur_level + 1,
                                    3 * (change_at_ind + i),
                                    Levels,
                                    dp,
                                ));
                            }
                        }
                    }

                    _ => unreachable!(),
                };

                mi
            };

            dp[cur_level][change_at_ind] = ans;

            ans
        }

        let mut dp = vec![vec![usize::MAX; S.len() + 1]; N + 1];
        let ans = solve(1, 0, &Levels, &mut dp);
        println!("{ans}");
    }
}
