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
    let t = cin.next::<i64>();

    for _ in 0..t {
        let k = cin.next::<i64>();
        let q = cin.next::<i64>();

        let mut a = vec![];
        for _ in 0..k {
            a.push(cin.next::<i64>());
        }

        let min = a.iter().min().unwrap();

        for _ in 0..q {
            let n = cin.next::<i64>();

            print!("{} ", n.min(min - 1));
        }
        println!("");
    }
}
