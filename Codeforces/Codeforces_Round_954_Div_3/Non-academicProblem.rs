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

pub fn get_bridges(adj: &[Vec<(usize, usize)>], m: usize) -> (usize, Vec<bool>, Vec<usize>) {
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        p_id: Option<usize>,
        adj: &[Vec<(usize, usize)>],
        timer: &mut usize,
        tin: &mut [usize],
        num_2_edge_ccs: &mut usize,
        is_bridge: &mut [bool],
        two_edge_ccid: &mut [usize],
        st: &mut Vec<usize>,
    ) -> usize {
        tin[u] = *timer;
        let (mut low, st_sz) = (*timer, st.len());
        *timer += 1;
        st.push(u);
        for &(v, e_id) in &adj[u] {
            if Some(e_id) == p_id {
                continue;
            }
            if tin[v] == 0 {
                low = low.min(dfs(
                    v,
                    Some(e_id),
                    adj,
                    timer,
                    tin,
                    num_2_edge_ccs,
                    is_bridge,
                    two_edge_ccid,
                    st,
                ));
            }
            low = low.min(tin[v]);
        }
        if tin[u] == low {
            if let Some(p) = p_id {
                is_bridge[p] = true;
            }
            for &id in st.iter().skip(st_sz) {
                two_edge_ccid[id] = *num_2_edge_ccs;
            }
            st.truncate(st_sz);
            *num_2_edge_ccs += 1;
        }
        low
    }
    let (n, mut timer, mut num_2_edge_ccs, mut is_bridge) = (adj.len(), 1, 0, vec![false; m]);
    let (mut tin, mut two_edge_ccid, mut st) = (vec![0; n], vec![0; n], Vec::with_capacity(n));
    for i in 0..n {
        if tin[i] == 0 {
            dfs(
                i,
                None,
                adj,
                &mut timer,
                &mut tin,
                &mut num_2_edge_ccs,
                &mut is_bridge,
                &mut two_edge_ccid,
                &mut st,
            );
        }
    }
    (num_2_edge_ccs, is_bridge, two_edge_ccid)
}

fn dfs(
    cur: usize,
    adj: &Vec<Vec<(usize, usize)>>,
    is_bridge: &Vec<Vec<bool>>,
    used: &mut Vec<bool>,
    ans: &mut usize,
) -> usize {
    used[cur] = true;

    let mut c = 1;
    for (ind, &(to, _)) in adj[cur].iter().enumerate() {
        if !used[to] {
            let cnt = dfs(to, adj, is_bridge, used, ans);
            c += cnt;
            if is_bridge[cur][ind] {
                *ans = (*ans).max(cnt * (adj.len() - cnt));
            }
        }
    }
    c
}

fn main() {
    let mut cin = Cin::new();

    let _t = cin.next::<usize>();
    for _ in 0.._t {
        let (n, m) = (cin.next::<usize>(), cin.next::<usize>());

        let mut adj = vec![vec![]; n];
        let mut edges = vec![];
        for i in 0..m {
            let (x, y) = (cin.next::<usize>(), cin.next::<usize>());
            adj[x - 1].push((y - 1, i));
            adj[y - 1].push((x - 1, i));

            edges.push((x - 1, y - 1));
        }

        let (_, is_bridge_ed, _) = get_bridges(&adj, m);

        let mut is_bridge = vec![vec![]; n];
        for i in 0..m {
            let (x, y) = edges[i];
            is_bridge[x].push(is_bridge_ed[i]);
            is_bridge[y].push(is_bridge_ed[i]);
        }

        let mut used = vec![false; n];
        let mut x = 0;
        let _ = dfs(0, &adj, &is_bridge, &mut used, &mut x);
        let ans = n * (n - 1) / 2 - x;

        println!("{ans}");
    }
}
