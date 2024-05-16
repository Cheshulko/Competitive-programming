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
        let (mut a, mut b, mut c) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut cur_level = (a as f64).log2() as usize;
        let used_a = (1 << cur_level) - 1;
        a -= used_a;

        let mut fill_current_level = 0;
        let mut fill_next_level = 1 << cur_level;

        while a > 0 || b > 0 || c > 0 {
            if fill_current_level == 0 {
                if fill_next_level == 0 {
                    break;
                }
                fill_current_level = fill_next_level;
                fill_next_level = 0;
                cur_level += 1;
            }

            while a > 0 && fill_current_level != 0 {
                fill_current_level -= 1;
                fill_next_level += 2;
                a -= 1;
            }

            while b > 0 && fill_current_level != 0 {
                fill_current_level -= 1;
                fill_next_level += 1;
                b -= 1;
            }

            while c > 0 && fill_current_level != 0 {
                fill_current_level -= 1;
                c -= 1;
            }
        }

        if fill_next_level == 0 && fill_current_level == 0 && a == 0 && b == 0 && c == 0 {
            cur_level -= 1;
            println!("{cur_level}");
        } else {
            println!("-1");
        }
    }
}
