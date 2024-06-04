use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

    'out: for _ in 0..t {
        let n = cin.next::<usize>();
        let s = cin.next::<String>();

        let mut nn = -1;
        let mut mm = (b'a' - 1) as i32;

        let mut cha = false;

        for c in s.into_bytes().into_iter() {
            let dig = c.is_ascii_digit();

            let c = c as i32;

            if !dig {
                cha = true;
            }

            if cha && dig {
                println!("NO");
                continue 'out;
            }

            if dig && nn > c {
                println!("NO");
                continue 'out;
            } else {
                nn = c;
            }

            if !dig && mm > c {
                println!("NO");
                continue 'out;
            } else {
                mm = c;
            }
        }

        println!("YES");
    }
}
