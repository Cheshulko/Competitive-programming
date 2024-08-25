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

fn dijkstra(vert: &Vec<usize>, adj: &Vec<Vec<(usize, usize)>>, s: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj.len()];
    dist[s] = vert[s];

    let mut visited = vec![false; adj.len()];
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((dist[s], s)));
    while let Some(Reverse((d, u))) = queue.pop() {
        if visited[u] {
            continue;
        }
        visited[u] = true;
        for &(v, w) in &adj[u] {
            if dist[v] > d + w + vert[v] {
                dist[v] = d + w + vert[v];
                queue.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

fn main() {
    let mut cin = Cin::new();

    let _t = 1;
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut a = vec![0; n];
        for i in 0..n {
            a[i] = cin.next::<usize>();
        }

        let mut adj = vec![vec![]; n];
        for _ in 0..m {
            let x = cin.next::<usize>();
            let y = cin.next::<usize>();
            let b = cin.next::<usize>();

            adj[x - 1].push((y - 1, b));
            adj[y - 1].push((x - 1, b));
        }

        let ans = dijkstra(&a, &adj, 0);
        let res = &ans[1..];
        for i in 0..res.len() {
            print!("{x} ", x = res[i]);
        }
        println!();
    }
}
