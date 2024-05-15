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
        let mut n = cin.next::<usize>();

        if n % 2 == 1 {
            println!("NO");
            continue;
        }

        let mut ans = String::new();

        let mut next = b'A';
        while n > 0 {
            ans.push(next as char);
            n -= 1;
            if ans.len() % 2 == 0 {
                next += 1;
            }
        }

        println!("YES");
        println!("{ans}");
    }
}
