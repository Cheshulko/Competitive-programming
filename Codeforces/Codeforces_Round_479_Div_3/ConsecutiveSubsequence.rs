use core::num;
use std::cmp::*;
use std::collections::*;
use std::i128;
use std::i32;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
use std::rc;
use std::slice::*;
use std::usize;
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
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let n = cin.next::<usize>();

        let mut arr = vec![0; n];
        for i in 0..n {
            arr[i] = cin.next::<usize>();
        }

        let mut indx = vec![None; n];
        let mut bt = HashMap::<usize, (usize, usize)>::new();
        for i in 0..n {
            let v = arr[i];

            if let Some(&(prev, prev_ind)) = bt.get(&(v - 1)) {
                let (cur, cur_ind) = bt.entry(v).or_insert((1, i));

                if (*cur) < prev + 1 {
                    *cur = prev + 1;
                    *cur_ind = i;
                    indx[i] = Some(prev_ind);
                }
            } else {
                bt.insert(v, (1, i));
                indx[i] = None;
            }
        }

        let mut ans = 0;
        let mut ans_ind = 0;
        for &(v, ind) in bt.values() {
            if v > ans {
                ans = v;
                ans_ind = ind;
            }
        }

        let mut ans_arr = vec![];
        loop {
            ans_arr.push(ans_ind + 1);

            if let Some(par_ind) = indx[ans_ind] {
                ans_ind = par_ind;
            } else {
                break;
            }
        }

        assert!(ans_arr.len() == ans);
        println!("{l}", l = ans_arr.len());
        for x in ans_arr.into_iter().rev() {
            print!("{x} ");
        }
        println!();
    }
}
