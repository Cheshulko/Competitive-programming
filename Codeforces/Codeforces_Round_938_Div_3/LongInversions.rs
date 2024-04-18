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

fn can(s: &[u8], buf: &mut [u8], k: usize) -> bool {
    let n = s.len();

    for i in 0..n {
        if (s[i] == b'0' && (buf[i] % 2 == 0)) || (s[i] == b'1' && (buf[i] % 2 != 0)) {
            if n - i >= k {
                for j in i..(i + k) {
                    buf[j] += 1;
                }
            } else {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let mut cin = Cin::new();

    let t = cin.next::<usize>();

    for _ in 0..t {
        let _ = cin.next::<usize>();
        let s = cin.next::<String>();

        for x in (0..=s.len()).rev() {
            let mut buf = vec![0; s.len()];
            let is_can = can(s.as_bytes(), &mut buf, x);

            if is_can {
                println!("{}", x);
                break;
            }
        }
    }
}
