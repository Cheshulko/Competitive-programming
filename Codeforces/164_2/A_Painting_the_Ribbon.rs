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
        let (n, m, k) = (cin.next::<i64>(), cin.next::<i64>(), cin.next::<i64>());

        let x = n / m + ((n % m != 0) as i64);
        let y = n - x;
        if k >= y {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
