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
        let (x1, x2, x3, x4) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut cnt = x1 + x2 + x3 + x4;

        cnt -= x4 % 2;

        let has_tripple = x1 % 2 + x2 % 2 + x3 % 2;

        cnt -= x4 / 2;
        cnt -= x1 / 2;
        cnt -= x2 / 2;
        cnt -= x3 / 2;

        cnt += has_tripple / 3;
        cnt -= has_tripple;

        println!("{}", cnt);
    }
}
