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
        let (N, M) = (cin.next::<usize>(), cin.next::<usize>());

        let mut S = vec![];
        for _ in 0..N {
            let s = cin.next::<String>().chars().collect::<Vec<_>>();
            S.push(s);
        }

        let mut T = vec![];
        for _ in 0..M {
            let s = cin.next::<String>().chars().collect::<Vec<_>>();
            T.push(s);
        }

        fn check(ii: usize, jj: usize, S: &Vec<Vec<char>>, T: &Vec<Vec<char>>) -> bool {
            let M = T.len();

            for i in ii..ii + M {
                for j in jj..jj + M {
                    if S[i][j] != T[i - ii][j - jj] {
                        return false;
                    }
                }
            }

            return true;
        }

        for i in 0..1 + N - M {
            for j in 0..1 + N - M {
                if check(i, j, &S, &T) {
                    println!("{} {}", i + 1, j + 1);
                    continue 'test;
                }
            }
        }
    }
}
