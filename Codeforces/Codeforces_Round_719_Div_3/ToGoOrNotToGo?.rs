use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
use std::i128;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::mem::swap;
use std::path::Path;
use std::usize;
use std::vec;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    const DIRS: &[(i32, i32)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

    let _t = 1;
    for _ in 0.._t {
        let (n, m, w) = (cin.next::<usize>(), cin.next::<usize>(), cin.next::<i128>());

        let mut grid = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                grid[i][j] = cin.next::<i128>();
            }
        }

        let bfs_from = |cur: (usize, usize)| -> (Vec<Vec<i128>>, i128) {
            let mut dist = vec![vec![i128::MAX; m]; n];
            dist[cur.0][cur.1] = 0;

            let mut best_portal = i128::MAX;

            let mut visited = vec![vec![false; m]; n];
            visited[cur.0][cur.1] = true;
            let mut queue = VecDeque::new();

            queue.push_back((0, cur));
            while let Some((d, (cur_i, cur_j))) = queue.pop_front() {
                if grid[cur_i][cur_j] > 0 && grid[cur_i][cur_j] + d < best_portal {
                    best_portal = grid[cur_i][cur_j] + d;
                }

                dist[cur_i][cur_j] = d;

                let x = DIRS
                    .iter()
                    .filter_map(|(di, dj)| {
                        let to_i = (cur_i as i32 + di) as usize;
                        let to_j = (cur_j as i32 + dj) as usize;
                        (grid.get(to_i)?.get(to_j)? != &-1).then_some((to_i, to_j))
                    })
                    .collect::<Vec<_>>();

                for (to_i, to_j) in x.into_iter() {
                    if !visited[to_i][to_j] {
                        visited[to_i][to_j] = true;

                        queue.push_back((d + w, (to_i, to_j)));
                    }
                }
            }

            return (dist, best_portal);
        };

        let (from_start_dist, best1) = bfs_from((0, 0));
        let (from_end_dist, best2) = bfs_from((n - 1, m - 1));

        assert!(from_end_dist[0][0] == from_start_dist[n - 1][m - 1]);

        let mut ans = from_end_dist[0][0];

        if best1 != i128::MAX && best2 != i128::MAX {
            ans = ans.min(best1 + best2)
        }

        if ans == i128::MAX {
            println!("-1");
        } else {
            println!("{ans}");
        }
    }

    Ok(())
}
