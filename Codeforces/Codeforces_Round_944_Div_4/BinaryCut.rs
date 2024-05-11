use std::cmp::*;
use std::collections::*;
use std::io::stdin;

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
        let x = cin.next::<String>();
        let x = x.chars().collect::<Vec<_>>();

        let mut a = 0;
        let mut b = 0;
        let n = x.len();

        for i in 0..(n - 1) {
            if x[i] == '0' && x[i + 1] == '1' {
                a += 1;
            }
            if x[i] == '1' && x[i + 1] == '0' {
                b += 1;
            }
        }

        if a >= 1 {
            a -= 1;
        }
        let ans = 1 + a + b;

        println!("{ans}");
    }
}
