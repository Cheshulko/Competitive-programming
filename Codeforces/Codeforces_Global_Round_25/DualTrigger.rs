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
        let _ = cin.next::<usize>();
        let s = cin.next::<String>();
        let x = s
            .char_indices()
            .filter_map(|(ind, c)| (c == '1').then_some(ind))
            .collect::<Vec<_>>();

        if x.len() % 2 == 1 || (x.len() == 2 && x[1] - x[0] == 1) {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}
