use std::cmp::*;
use std::collections::*;
use std::error::Error;
use std::fs::File;
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

fn dijkstra(adj: &Vec<Vec<(usize, usize)>>, aa: &Vec<bool>, s: usize) -> Vec<usize> {
    let mut dist = vec![usize::MAX; adj.len()];
    dist[s] = 0;

    let mut visited = vec![0; adj.len()];
    let mut queue = BinaryHeap::new();

    queue.push(Reverse((0, s, 0)));
    while let Some(Reverse((d, u, has))) = queue.pop() {
        let has = has.max(aa[u] as usize);

        if visited[u] & (1 << has) > 0 {
            continue;
        }
        visited[u] |= (1 << has);

        for &(v, w) in &adj[u] {
            let mut w = w;
            if has > 0 {
                w = w / 2;
            }
            if dist[v] > d + w {
                dist[v] = d + w;
                queue.push(Reverse((dist[v], v, has)));
            } else if visited[v] & (1 << has) == 0 {
                queue.push(Reverse((d + w, v, has)));
            }
        }
    }
    dist
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m, h) = (
            cin.next::<usize>(),
            cin.next::<usize>(),
            cin.next::<usize>(),
        );

        let mut a = vec![0; n];
        let mut aa = vec![false; n];
        for i in 0..h {
            a[i] = cin.next::<usize>();
            aa[a[i] - 1] = true;
        }

        let mut adj = vec![vec![]; n];
        for _ in 0..m {
            let (v, u, w) = (
                cin.next::<usize>(),
                cin.next::<usize>(),
                cin.next::<usize>(),
            );

            let v = v - 1;
            let u = u - 1;
            adj[v].push((u, w));
            adj[u].push((v, w));
        }

        let d1 = dijkstra(&adj, &aa, 0);
        let d2 = dijkstra(&adj, &aa, n - 1);
        let mut ans = usize::MAX;
        for i in 0..n {
            if d1[i] != usize::MAX && d2[i] != usize::MAX {
                ans = ans.min(d1[i].max(d2[i]));
            }
        }
        if ans != usize::MAX {
            println!("{ans}");
        } else {
            println!("-1");
        }
    }

    Ok(())
}
