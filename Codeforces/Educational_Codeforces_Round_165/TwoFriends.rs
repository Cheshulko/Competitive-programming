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
        let n = cin.next::<usize>();

        let mut arr = vec![];

        for _ in 0..n {
            let x = cin.next::<usize>();
            arr.push(x - 1);
        }

        let mut ans = usize::MAX;
        for i in 0..n {
            let mut x = 0;

            let mut cur = i;

            loop {
                cur = arr[cur];
                x += 1;

                if cur == i {
                    break;
                }
            }

            ans = ans.min(x);
        }

        ans = ans.min(3);

        println!("{ans}");
    }
}
