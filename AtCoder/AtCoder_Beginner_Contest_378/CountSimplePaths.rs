use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;

struct Cin {
    reader: Box<dyn std::io::BufRead>,
    tokens: VecDeque<String>,
}

impl Cin {
    pub fn file(path: &Path) -> Self {
        let tokens = VecDeque::new();
        let file = File::open(&path).expect("Expect file exists");
        Self {
            reader: Box::new(BufReader::new(file)),
            tokens,
        }
    }
    pub fn new() -> Self {
        let tokens = VecDeque::new();
        Self {
            reader: Box::new(BufReader::new(std::io::stdin())),
            tokens,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        if self.tokens.is_empty() {
            let mut buffer = String::new();
            self.reader.read_line(&mut buffer).unwrap();
            for s in buffer.split_whitespace() {
                self.tokens.push_back(s.to_string());
            }
        }
        let fr = self.tokens.pop_front().unwrap_or(String::new());
        fr.parse::<T>().ok().unwrap()
    }
}

fn dfs(
    cur_i: usize,
    cur_j: usize,
    g: &Vec<Vec<usize>>,
    used: &mut Vec<Vec<bool>>,
    cnt: usize,
    k: usize,
) -> usize {
    if cnt == k {
        return 1;
    }

    used[cur_i][cur_j] = true;
    let d = [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(|(di, dj)| {
            let to_i = (cur_i as i32 + di) as usize;
            let to_j = (cur_j as i32 + dj) as usize;
            (g.get(to_i)?.get(to_j)? != &0).then_some((to_i, to_j))
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for (to_i, to_j) in d.into_iter() {
        if !used[to_i][to_j] {
            ans += dfs(to_i, to_j, g, used, cnt + 1, k)
        }
    }

    used[cur_i][cur_j] = false;

    ans
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = 1;
    // let _t = cin.next::<usize>();
    #[allow(unused_labels)]
    'test: for _ in 0.._t {
        let (h, w, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut g = vec![];
        for i in 0..h {
            let s = cin.next::<String>().into_bytes();
            let s = s
                .into_iter()
                .map(|x| (x == b'.') as usize)
                .collect::<Vec<_>>();
            g.push(s);
        }

        let mut ans = 0;
        for i in 0..h {
            for j in 0..w {
                if g[i][j] == 1 {
                    let mut used = vec![vec![false; w]; h];
                    ans += dfs(i, j, &g, &mut used, 0, k);
                }
            }
        }

        println!("{ans}");
    }

    Ok(())
}
