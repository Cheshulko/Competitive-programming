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
    let t = cin.next::<i32>();

    for _ in 0..t {
        let (n, a, b) = (cin.next::<i32>(), cin.next::<i32>(), cin.next::<i32>());

        if b >= 2 * a {
            println!("{}", n * a);
        } else {
            let ans = (n / 2) * b + (n % 2) * a;
            println!("{}", ans);
        }
    }
}
