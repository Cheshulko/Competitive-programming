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
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let size = k;

        let mut a = vec![0; n];
        let mut clique = vec![0; n];
        let mut i = 0;
        let mut clique_cnt = 1;
        while i < n {
            let mid = (clique_cnt * size - (size / 2)).min(n);
            let mut cur = mid;
            for j in i..(i + size) {
                if j >= n {
                    break;
                }
                clique[j] = clique_cnt;
                a[j] = cur;
                cur -= 1;
                if cur == ((clique_cnt - 1) * size) {
                    cur = (clique_cnt * size).min(n);
                }
            }
            i = i + size;
            clique_cnt += 1;
        }

        clique_cnt -= 1;

        for x in a.into_iter() {
            print!("{x} ");
        }
        println!();
        println!("{clique_cnt}");
        for x in clique.into_iter() {
            print!("{x} ");
        }
        println!();
    }
}
