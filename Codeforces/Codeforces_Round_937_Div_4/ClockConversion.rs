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
        let n = cin.next::<String>();
        let mut m = n.split(":");
        let a = m.next().unwrap().parse::<usize>().unwrap();
        let b = m.next().unwrap();

        let am = a / 12 == 0;
        let mut a = a % 12;
        if a == 0 {
            a = 12;
        }

        println!("{a:02}:{b} {x}", x = if am { "AM" } else { "PM" });
    }
}
