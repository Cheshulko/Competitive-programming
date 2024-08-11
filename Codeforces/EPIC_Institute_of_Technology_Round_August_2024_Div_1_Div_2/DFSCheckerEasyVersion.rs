use core::num;
use std::cmp::*;
use std::collections::*;
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

fn build(
    cur: usize,
    p: &Vec<usize>,
    par: &Vec<i32>,
    step: usize,
    cur_dep: usize,
    dep: usize,
    out_ch: &mut Vec<Vec<usize>>,
    out_pa: &mut Vec<i32>,
) {
    if cur_dep == dep {
        return;
    }

    let mut ch = cur + 1;

    for _ in 0..2 {
        out_pa[ch] = cur as i32;
        out_ch[cur].push(ch);

        build(ch, p, par, step / 2, cur_dep + 1, dep, out_ch, out_pa);
        ch += step;
    }
}

fn main() {
    let mut cin = Cin::new();

    // let _t = 1;
    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, q) = (cin.next::<usize>(), cin.next::<usize>());

        let dep = (1 + n).ilog2() as usize;

        let mut par = vec![-1; n];
        for i in 0..n - 1 {
            let x = cin.next::<i32>();
            par[i + 1] = x - 1;
        }

        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = cin.next::<usize>();
            p[i] -= 1;
        }

        let mut out_ch = vec![vec![]; n];
        let mut out_pa = vec![-1; n];
        build(0, &p, &par, n / 2, 1, dep, &mut out_ch, &mut out_pa);

        let mut broken = (0..n).collect::<HashSet<_>>();
        for _ in 0..q {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());

            let x = x - 1;
            let y = y - 1;

            p.swap(x, y);

            broken.insert(x);
            broken.insert(y);

            if out_pa[x] != -1 {
                broken.insert(out_pa[x] as usize);
            }
            if out_pa[y] != -1 {
                broken.insert(out_pa[y] as usize);
            }
            if out_ch[x].len() > 0 {
                broken.insert(out_ch[x][0] as usize);
                broken.insert(out_ch[x][1] as usize);
            }
            if out_ch[y].len() > 0 {
                broken.insert(out_ch[y][0] as usize);
                broken.insert(out_ch[y][1] as usize);
            }

            let mut fixed = HashSet::<usize>::new();

            for &x in broken.iter() {
                let mut ch_k = true;
                if out_ch[x].len() > 0 {
                    let ch1 = out_ch[x][0];
                    let ch2 = out_ch[x][1];

                    if p[x] as i32 == par[p[ch1] as usize] && p[x] as i32 == par[p[ch2] as usize] {
                    } else {
                        ch_k = false;
                    }
                }
                let mut pa_k = true;
                if out_pa[x] != -1 {
                    let pa = out_pa[x] as usize;

                    if par[p[x] as usize] == p[pa] as i32 {
                    } else {
                        pa_k = false;
                    }
                }

                if pa_k && ch_k {
                    fixed.insert(x);
                }
            }

            for x in fixed.into_iter() {
                broken.remove(&x);
            }

            if broken.is_empty() {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}
