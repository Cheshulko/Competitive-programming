// https://leetcode.com/problems/power-grid-maintenance

struct Solution {}

impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeSet;

        let n = c as usize;

        let connections = connections.into_iter().fold(vec![vec![]; n], |mut v, x| {
            let a = x[0] as usize - 1;
            let b = x[1] as usize - 1;
            v[a].push(b);
            v[b].push(a);
            v
        });

        let mut seen = vec![false; n];
        let mut components = vec![];
        let mut in_component = vec![0; n];
        let mut on = vec![true; n];

        fn dfs(
            cur: usize,
            ind: usize,
            connections: &Vec<Vec<usize>>,
            seen: &mut Vec<bool>,
            component: &mut BTreeSet<usize>,
            in_component: &mut Vec<usize>,
        ) {
            component.insert(cur);
            seen[cur] = true;
            in_component[cur] = ind;

            for &to in connections[cur].iter() {
                if !seen[to] {
                    dfs(to, ind, connections, seen, component, in_component);
                }
            }
        }

        for i in 0..n {
            if !seen[i] {
                let mut component = BTreeSet::new();
                dfs(
                    i,
                    components.len(),
                    &connections,
                    &mut seen,
                    &mut component,
                    &mut in_component,
                );
                components.push(component);
            }
        }

        let mut ans = vec![];
        for q in queries {
            let t = q[0];
            let x = q[1] as usize - 1;
            match t {
                1 => {
                    if on[x] {
                        ans.push(x as i32 + 1);
                    } else {
                        let ind = in_component[x];
                        let v = components[ind]
                            .first()
                            .copied()
                            .map(|x| x as i32)
                            .unwrap_or(-2);

                        ans.push(v + 1);
                    }
                }
                2 => {
                    on[x] = false;
                    let ind = in_component[x];
                    components[ind].remove(&x);
                }
                _ => unreachable!(),
            }
        }

        ans
    }
}
