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
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<String>();
            let x = x.into_bytes();
            arr.push(x);
        }

        let mut b = 0;
        let mut w = 0;

        for i in 0..n {
            if arr[i][0] == b'B' {
                b += 1;
                break;
            }
        }
        for i in 0..n {
            if arr[i][m - 1] == b'B' {
                b += 1;
                break;
            }
        }
        for j in 0..m {
            if arr[0][j] == b'B' {
                b += 1;
                break;
            }
        }
        for j in 0..m {
            if arr[n - 1][j] == b'B' {
                b += 1;
                break;
            }
        }

        //--
        for i in 0..n {
            if arr[i][0] == b'W' {
                w += 1;
                break;
            }
        }
        for i in 0..n {
            if arr[i][m - 1] == b'W' {
                w += 1;
                break;
            }
        }
        for j in 0..m {
            if arr[0][j] == b'W' {
                w += 1;
                break;
            }
        }
        for j in 0..m {
            if arr[n - 1][j] == b'W' {
                w += 1;
                break;
            }
        }

        if b == 4 || w == 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
