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
        let x = cin.next::<String>();
        let y = cin.next::<String>();

        let mut x_ = x.chars().collect::<Vec<_>>();
        let mut y_ = y.chars().collect::<Vec<_>>();

        let n = x.len();

        let mut gr = -1;
        for i in 0..n {
            if x_[i] == y_[i] {
                continue;
            }
            if x_[i] > y_[i] {
                if gr == -1 {
                    gr = 1;
                } else if gr == 1 {
                    let t = x_[i];
                    x_[i] = y_[i];
                    y_[i] = t;
                } else if gr == 2 {
                    //.
                }
            } else {
                if gr == -1 {
                    gr = 2;
                } else if gr == 2 {
                    let t = x_[i];
                    x_[i] = y_[i];
                    y_[i] = t;
                } else if gr == 1 {
                    //.
                }
            }
        }

        let x = x_.into_iter().collect::<String>();
        let y = y_.into_iter().collect::<String>();

        println!("{x}\n{y}");
    }
}
