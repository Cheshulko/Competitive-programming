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
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<u64>();
            arr.push(x);
        }

        let mut pref = vec![0; n + 1];
        let mut nums_at_pos = HashMap::<u64, Vec<usize>>::new();

        let mut cnt = vec![vec![0; 30]; n + 1];
        for i in 1..=n {
            for bit in 0..30 {
                cnt[i][bit] = cnt[i - 1][bit] + ((1 << bit) & arr[i - 1] > 0) as u64;
            }

            pref[i] = pref[i - 1] ^ arr[i - 1];
            nums_at_pos.entry(pref[i]).or_default().push(i - 1);
        }

        let nums_at_pos = dbg!(nums_at_pos);

        for _ in 0..q {
            let (l, r) = (cin.next::<usize>(), cin.next::<usize>());

            let mut bits = cnt[r].clone();
            for b in 0..30 {
                bits[b] -= cnt[l - 1][b];
            }

            let odds = bits
                .iter()
                .enumerate()
                .filter_map(|(b, &cnt)| (cnt % 2 == 1).then_some(b))
                .collect::<Vec<_>>();
            let before = pref[l - 1];

            if odds.is_empty() {
                if let Some(positions) = nums_at_pos.get(&before) {
                    let p = match positions.binary_search(&(l - 1)) {
                        Ok(p) => p,
                        Err(p) => p,
                    };
                    if p == positions.len() {
                        println!("NO");
                        continue;
                    }
                    if positions[p] < r {
                        println!("YES");
                    } else {
                        println!("NO");
                    }
                } else {
                    println!("NO");
                }
            } else {
                let mut first = before;
                for &b in &odds {
                    first ^= 1 << b;
                }

                let mut second = first;
                for b in odds {
                    second ^= 1 << b;
                }

                let first_position = if let Some(positions) = nums_at_pos.get(&first) {
                    let p = match positions.binary_search(&(l - 1)) {
                        Ok(p) => p,
                        Err(p) => p,
                    };
                    if p == positions.len() {
                        println!("NO");
                        continue;
                    }
                    let p = positions[p];
                    (p < r).then_some(p)
                } else {
                    None
                };

                if let Some(first_position) = first_position {
                    if let Some(positions) = nums_at_pos.get(&second) {
                        let p = match positions.binary_search(&first_position) {
                            Ok(p) => p,
                            Err(p) => p,
                        };
                        if p == positions.len() {
                            println!("NO");
                            continue;
                        }
                        if positions[p] < r {
                            println!("YES");
                        } else {
                            println!("NO");
                        }
                    } else {
                        println!("NO");
                    }
                } else {
                    println!("NO");
                }
            }
        }
    }
}
