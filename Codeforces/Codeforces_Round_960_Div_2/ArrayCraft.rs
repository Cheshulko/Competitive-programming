use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
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
    // let _t = 1;
    for _ in 0.._t {
        let (n, x, y) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut arr = vec![0; n];

        for i in (y - 1)..x {
            arr[i] = 1;
        }

        let mut u = true;
        for i in (0..y).rev() {
            if u {
                arr[i] = 1;
            } else {
                arr[i] = -1;
            }
            u = !u;
        }

        let mut u = true;
        for i in (x - 1)..n {
            if u {
                arr[i] = 1;
            } else {
                arr[i] = -1;
            }
            u = !u;
        }

        for x in arr.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
