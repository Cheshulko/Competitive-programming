use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn can(arr: &Vec<usize>, tar: usize) -> bool {
    let n = arr.len();

    let mut have = 0;
    for i in 0..n {
        if arr[i] > tar {
            have += arr[i] - tar;
        } else if arr[i] < tar {
            let need = tar - arr[i];
            if have >= need {
                have -= need;
            } else {
                return false;
            }
        }
    }

    return true;
}

fn can2(arr: &Vec<usize>, tar: usize) -> bool {
    let n = arr.len();

    let mut extra = 0;
    for i in 0..n {
        if arr[i] > tar {
            extra += arr[i] - tar;
        } else if arr[i] < tar {
            let need = tar - arr[i];
            if extra >= need {
                extra -= need;
            } else {
                extra = 0;
            }
        }
    }

    return extra == 0;
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = cin.next::<usize>();
        }

        let mut l = 0;
        let mut r = arr.iter().max().unwrap() + 1;
        while r - l > 1 {
            let m = (l + r) >> 1;

            let c = can(&arr, m);
            if c {
                l = m;
            } else {
                r = m;
            }
        }

        let l1 = l;

        let mut l = 0;
        let mut r = arr.iter().max().unwrap() + 1;
        while r - l > 1 {
            let m = (l + r) >> 1;

            let c = can2(&arr, m);
            if c {
                r = m;
            } else {
                l = m;
            }
        }

        let r1 = r;

        println!("{ans}", ans = r1 - l1);
    }
}
