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
        let r = cin.next::<i64>();

        let mut ans = 0;
        let mut cur = r;
        let mut dr = 0;
        loop {
            if cur * cur + dr * dr < (r + 1) * (r + 1) {
                if cur * cur + dr * dr >= r * r {
                    ans += 1;
                }
                dr += 1;
            } else {
                cur -= 1;
                dr -= 1;
            }

            if cur < 0 {
                break;
            }
        }
        ans *= 4;
        ans -= 4;

        println!("{ans}");
    }
}
