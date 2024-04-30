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
        // let n = cin.next::<usize>();
        // let mut arr = vec![];

        let x = cin.next::<String>();
        let x = x.as_bytes();
        let len = x.len();

        let mut p1 = -1;
        let mut ans = 0;
        let mut i = 0;

        while i < len {
            let c = x[i];

            if c == b'1' {
                if p1 == -1 {
                    p1 = i as i64;
                }
            } else {
                if p1 != -1 {
                    let left = i as i64 - p1 + 1;
                    ans += left;
                    p1 += 1;
                }
            }
            i += 1;
        }

        println!("{ans}");
    }
}
