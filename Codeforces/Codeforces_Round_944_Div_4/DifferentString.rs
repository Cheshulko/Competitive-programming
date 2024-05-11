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
        let m = x.chars().collect::<HashSet<_>>();

        if m.len() == 1 {
            println!("NO");
        } else {
            println!("YES");
            let y = x.chars().next().unwrap();
            let mut z = x.chars().skip(1).collect::<Vec<_>>();
            z.push(y);
            let x = z.into_iter().collect::<String>();
            println!("{x}");
        }
    }
}
