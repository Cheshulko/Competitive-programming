use std::collections::BinaryHeap;

mod cm_topological {
    fn dfs_topological(
        cur: usize,
        edges: &Vec<Vec<usize>>,
        used: &mut Vec<bool>,
        ans: &mut Vec<usize>,
    ) {
        used[cur] = true;
        for to in &edges[cur] {
            if !used[*to] {
                dfs_topological(*to, edges, used, ans);
            }
        }
        ans.push(cur);
    }

    pub fn topological_sort(n: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];
        let mut used = vec![false; n];

        for to in 0..n {
            if !used[to] {
                dfs_topological(to, edges, &mut used, &mut ans);
            }
        }
        ans.reverse();

        ans
    }

    pub fn check_is_topological(n: usize, edges: &Vec<Vec<usize>>, order: &Vec<usize>) -> bool {
        let mut pos = vec![0; n];
        for i in 0..n {
            pos[order[i]] = i;
        }

        for i in 0..n {
            for &to in edges[i].iter() {
                if pos[i] > pos[to] {
                    return false;
                }
            }
        }

        return true;
    }
}

impl Solution {
    pub fn max_candies(
        status: Vec<i32>,
        candies: Vec<i32>,
        keys: Vec<Vec<i32>>,
        contained_boxes: Vec<Vec<i32>>,
        initial_boxes: Vec<i32>,
    ) -> i32 {
        let n = candies.len();

        let status = status.into_iter().map(|x| x == 1).collect::<Vec<_>>();
        let keys = keys
            .into_iter()
            .map(|x| x.into_iter().map(|y| y as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let keys_tp = cm_topological::topological_sort(n, &keys);
        let mut keys_tp = keys_tp
            .into_iter()
            .enumerate()
            .fold(vec![0; n], |mut ks, (i, k)| {
                ks[k] = n - i;
                ks
            });

        for i in 0..n {
            if status[i] {
                keys_tp[i] = usize::MAX;
            }
        }

        let mut ans = 0;
        let mut boxes = BinaryHeap::new();
        for b in initial_boxes.into_iter() {
            let b = b as usize;

            boxes.push((keys_tp[b], b));
        }

        let mut have_keys = vec![0; n];
        while let Some((_, b)) = boxes.pop() {
            if status[b] || have_keys[b] > 0 {
                if !status[b] {
                    have_keys[b] -= 1;
                }
                ans += candies[b];
                for &to_b in contained_boxes[b].iter() {
                    let to_b = to_b as usize;

                    boxes.push((keys_tp[to_b], to_b));
                }
                for &to_k in keys[b].iter() {
                    have_keys[to_k] += 1;
                }
            }
        }

        ans
    }
}
