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

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != 0 {
        if a < b {
            swap(&mut a, &mut b);
        }
        a %= b;
    }
    b
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (mut x, y, mut k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        k -= 1;
        x += 1;
        while x % y == 0 {
            x /= y;
        }

        let mut need = y - (x % y);
        while x != 1 {
            if need > k {
                break;
            } else {
                k -= need;
                x += need;

                while x % y == 0 {
                    x /= y;
                }

                need = y - (x % y);
            }
        }

        x += k % (y - 1);
        println!("{x}");
    }
}
