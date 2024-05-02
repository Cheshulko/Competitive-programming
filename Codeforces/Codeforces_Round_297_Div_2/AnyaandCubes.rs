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
    // let t = cin.next::<usize>();
    let t = 1;

    for _ in 0..t {
        let (n, k, s) = (cin.next::<usize>(), cin.next::<u32>(), cin.next::<u128>());

        let mut arr = vec![];
        for _ in 0..n {
            let x = cin.next::<u128>();
            arr.push(x);
        }

        let mut can = vec![false; n];
        let mut f = vec![0; n];

        for i in 0..n {
            let x = arr[i];

            let mut cur = 1;
            let mut ok = true;
            for i in 1..=x {
                cur *= i;

                if cur > s {
                    ok = false;
                    break;
                }
            }

            if ok {
                can[i] = true;
                f[i] = cur;
            }
        }

        let half = n / 2;

        let mut arr_1: Vec<(u128, u32)> = vec![(0, 0)];
        for i in 0..half {
            let n = arr_1.len();
            for j in 0..n {
                let s1 = arr_1[j].0 + arr[i];
                if s1 <= s {
                    arr_1.push((s1, arr_1[j].1));
                }
                if can[i] {
                    let s2 = arr_1[j].0 + f[i];
                    let k2 = arr_1[j].1 + 1;
                    if s2 <= s && k2 <= k {
                        arr_1.push((s2, k2));
                    }
                };
            }
        }

        let mut arr_2: Vec<(u128, u32)> = vec![(0, 0)];
        for i in half..n {
            let n = arr_2.len();
            for j in 0..n {
                let s1 = arr_2[j].0 + arr[i];
                if s1 <= s {
                    arr_2.push((s1, arr_2[j].1));
                }
                if can[i] {
                    let s2 = arr_2[j].0 + f[i];
                    let k2 = arr_2[j].1 + 1;
                    if s2 <= s && k2 <= k {
                        arr_2.push((s2, k2));
                    }
                };
            }
        }

        let mut arr_2_hm = arr_2
            .into_iter()
            .fold(HashMap::<u128, Vec<u32>>::new(), |mut hm, x| {
                hm.entry(x.0).or_insert(vec![]).push(x.1);
                hm
            });

        for (_, x) in arr_2_hm.iter_mut() {
            x.sort_unstable();
        }

        let mut ans = 0;

        for x in arr_1.into_iter() {
            let need = s - x.0;
            let have_k = x.1;

            if let Some(y) = arr_2_hm.get(&need) {
                let p = y.partition_point(|&u| u <= k - have_k);
                ans += p;
            }
        }

        println!("{ans}");
    }
}
