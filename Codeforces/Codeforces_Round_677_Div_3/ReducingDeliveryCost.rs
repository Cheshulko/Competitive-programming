use std::cmp::*;
use std::collections::*;
use std::io::{stdin, stdout, Write};
use std::mem::swap;
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

fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj.len()];
    dist[s] = 0;

    let mut visited = vec![false; adj.len()];
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, s)));
    while let Some(Reverse((d, u))) = queue.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &(v, w) in &adj[u] {
            if dist[v] > d + w {
                dist[v] = d + w;
                queue.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

fn main() {
    let mut cin = Cin::new();

    // let _t = cin.next::<usize>();
    let _t = 1;

    for _ in 0.._t {
        let (n, m, k) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut adj = vec![vec![]; n];
        let mut edges = vec![];
        for _ in 0..m {
            let (x, y, w) = (
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
            );
            adj[x - 1].push((y - 1, w));
            adj[y - 1].push((x - 1, w));
            edges.push((x - 1, y - 1));
        }

        let mut routes = vec![];
        for _ in 0..k {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            routes.push((x - 1, y - 1));
        }

        let mut min_dists = vec![];
        for i in 0..n {
            min_dists.push(dijkstra(&adj, i));
        }

        let mut ans = 0;
        for &(x, y) in routes.iter() {
            assert!(min_dists[x][y] == min_dists[y][x]);
            ans += min_dists[x][y];
        }

        for &(i, j) in edges.iter() {
            let mut lans = 0;
            for &(x, y) in routes.iter() {
                lans += min_dists[x][y]
                    .min(min_dists[x][i] + min_dists[j][y])
                    .min(min_dists[x][j] + min_dists[i][y]);
            }
            ans = ans.min(lans);
        }

        println!("{ans}");
    }
}
