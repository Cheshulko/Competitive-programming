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
    let t = 1;

    const DIRS: &[(i32, i32)] = &[(1, 0), (0, -1), (-1, 0), (0, 1), (1, 0)];

    for _ in 0..t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut qu = VecDeque::<(i32, i32)>::new();

        let mut mtx = vec![];
        for i in 0..n {
            let s = cin.next::<String>().into_bytes();
            for j in 0..s.len() {
                if s[j] == b'.' {
                    qu.push_back((i as i32, j as i32));
                }
            }
            mtx.push(s);
        }

        let (n, m) = (n as i32, m as i32);

        while let Some((i, j)) = qu.pop_front() {
            for d in DIRS.windows(2) {
                let mut acc_i = i;
                let mut acc_j = j;

                let mut ok = true;
                for &(di, dj) in d {
                    let to_i = i + di;
                    let to_j = j + dj;
                    if to_i < n
                        && to_i >= 0
                        && to_j < m
                        && to_j >= 0
                        && mtx[to_i as usize][to_j as usize] == b'.'
                    {
                        acc_i += di;
                        acc_j += dj;
                    } else {
                        ok = false;
                    }
                }

                if ok {
                    if mtx[acc_i as usize][acc_j as usize] == b'*' {
                        mtx[acc_i as usize][acc_j as usize] = b'.';

                        qu.push_back((acc_i, acc_j));
                        for &(di, dj) in DIRS.iter().skip(1) {
                            let to_i = acc_i + di;
                            let to_j = acc_j + dj;
                            if to_i < n
                                && to_i >= 0
                                && to_j < m
                                && to_j >= 0
                                && mtx[to_i as usize][to_j as usize] == b'.'
                            {
                                qu.push_back((to_i, to_j));
                            }
                        }
                    }
                }
            }
        }

        for row in mtx.into_iter() {
            let s = std::str::from_utf8(&row).unwrap();
            println!("{s}");
        }
        println!();
    }
}
