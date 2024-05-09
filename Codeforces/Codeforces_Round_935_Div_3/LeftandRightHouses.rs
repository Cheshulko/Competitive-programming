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

        let arr = cin
            .next::<String>()
            .chars()
            .map(|x| (x == '1') as usize)
            .collect::<Vec<_>>();

        let mut ok_right = arr.iter().sum::<usize>();
        let mut ok_left = 0;

        let is_ok = move |i: usize, ok_right: usize, ok_left: usize| -> bool {
            let r = n - i;
            let l = i;
            let hr = r / 2 + r % 2;
            let hl = l / 2 + l % 2;
            ok_right >= hr && ok_left >= hl
        };

        let (mut p, mut d) = if is_ok(0, ok_right, ok_left) {
            (0, n / 2)
        } else {
            (0, n + 1)
        };

        for i in 0..n {
            ok_left += (arr[i] == 0) as usize;
            ok_right -= (arr[i] == 1) as usize;

            if is_ok(i + 1, ok_right, ok_left) {
                let x1 = (n / 2).abs_diff(i + 1);
                let x2 = (n / 2 + n % 2).abs_diff(i + 1);
                if x1 < d {
                    p = i + 1;
                    d = x1;
                }
                if x2 < d {
                    p = i + 1;
                    d = x2;
                }
            }
        }

        println!("{p}");
    }
}
