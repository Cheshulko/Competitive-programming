use std::cmp::*;
use std::collections::*;
use std::io::stdin;
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
    let t = cin.next::<usize>();

    for _ in 0..t {
        let (mut x, y) = (cin.next::<usize>(), cin.next::<usize>());

        let mut yy = y / 2;
        let mut left = 7 * yy;
        if y % 2 == 1 {
            yy += 1;
            left += 7 + 4;
        }

        if left >= x {
        } else {
            x -= left;
            yy += x / 15;
            if x % 15 > 0 {
                yy += 1;
            }
        }

        println!("{yy}");
    }
}
