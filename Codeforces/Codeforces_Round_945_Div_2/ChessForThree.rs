use std::cmp::*;
use std::collections::*;
use std::io::stdin;
use std::slice::*;

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
        let (p1, p2, p3) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut arr = vec![p1, p2, p3];

        let mut ans = 0;

        loop {
            arr.sort();

            if arr[1] > 0 && arr[2] > 0 {
                ans += 1;
                arr[1] -= 1;
                arr[2] -= 1;
            } else {
                break;
            }
        }

        if arr[2] % 2 == 0 {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }
}
