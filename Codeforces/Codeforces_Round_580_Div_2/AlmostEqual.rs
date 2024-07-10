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

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        if n % 2 == 0 {
            println!("NO");
        } else {
            println!("YES");
            let mut arr = vec![0; 2 * n];
            let mut l = 1;
            let mut r = 2 * n;

            for i in 0..n {
                if i % 2 == 0 {
                    arr[i] = l;
                    l += 1;
                    arr[i + n] = l;
                    l += 1;
                } else {
                    arr[i] = r;
                    r -= 1;
                    arr[i + n] = r;
                    r -= 1;
                }
            }

            for x in arr.into_iter() {
                print!("{x} ");
            }
            println!();
        }
    }
}
