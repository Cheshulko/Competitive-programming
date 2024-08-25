use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::slice::*;
use std::vec;

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

fn unique_permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<T>> = vec![];

        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = items.clone();

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in unique_permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

fn is_pal(a: &[u8]) -> bool {
    let n = a.len();
    for i in 0..n / 2 {
        if a[i] != a[n - 1 - i] {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;
    for _ in 0.._t {
        let (n, k) = (cin.next::<usize>(), cin.next::<usize>());

        let mut s = cin.next::<String>().into_bytes();
        let mut ch = vec![0; 30];

        let pp = unique_permutations(s);
        let mut cnt = 0;
        for p in pp.into_iter() {
            let mut ok = true;
            for i in 0..(n - k + 1) {
                if is_pal(&p[i..(i + k)]) {
                    ok = false;
                    break;
                }
            }
            if ok {
                cnt += 1;
            }
        }
        println!("{cnt}");
    }
}
