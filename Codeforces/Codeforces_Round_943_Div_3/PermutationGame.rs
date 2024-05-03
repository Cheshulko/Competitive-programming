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
        let (n, k) = (cin.next::<usize>(), cin.next::<i128>());
        let (b, s) = (cin.next::<usize>(), cin.next::<usize>());

        let mut p = vec![0];
        for i in 0..n {
            let x = cin.next::<usize>();
            p.push(x);
        }

        let mut a = vec![0];
        let mut mx = 1;
        for i in 0..n {
            let x = cin.next::<i128>();
            a.push(x);
            mx = mx.max(x);
        }

        let mut best_b = 0;

        let mut used = vec![false; n + 1];
        let mut kk = k;
        let mut cur_b = 0;
        let mut cur_p_b = b;
        while kk > 0 {
            if used[cur_p_b] {
                break;
            }
            used[cur_p_b] = true;
            if a[cur_p_b] == mx {
                best_b = best_b.max(cur_b + kk * a[cur_p_b]);
                break;
            } else {
                best_b = best_b.max(cur_b + kk * a[cur_p_b]);
                cur_b += a[cur_p_b];
                cur_p_b = p[cur_p_b];
                kk -= 1;
            }
        }

        let mut best_s = 0;

        let mut used = vec![false; n + 1];
        let mut kk = k;
        let mut cur_s = 0;
        let mut cur_p_s = s;
        while kk > 0 {
            if used[cur_p_s] {
                break;
            }
            used[cur_p_s] = true;
            if a[cur_p_s] == mx {
                best_s = best_s.max(cur_s + kk * a[cur_p_s]);
                break;
            } else {
                best_s = best_s.max(cur_s + kk * a[cur_p_s]);
                cur_s += a[cur_p_s];
                cur_p_s = p[cur_p_s];
                kk -= 1;
            }
        }

        if best_b > best_s {
            println!("Bodya");
        } else if best_b < best_s {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}
