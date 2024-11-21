// https://leetcode.com/problems/count-unguarded-cells-in-the-grid

use std::collections::BTreeMap;

struct Solution {}

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (m as usize, n as usize);

        let mut rows = vec![BTreeMap::<usize, usize>::new(); n];
        let mut cols = vec![BTreeMap::<usize, usize>::new(); m];

        for guard in guards.iter() {
            let (x, y) = (guard[0] as usize, guard[1] as usize);
            rows[x].insert(y, 1);
            cols[y].insert(x, 1);
        }
        for wall in walls.into_iter() {
            let (x, y) = (wall[0] as usize, wall[1] as usize);
            rows[x].insert(y, 2);
            cols[y].insert(x, 2);
        }
        for i in 0..n {
            for j in 0..m {
                if !rows[i].contains_key(&j) {
                    rows[i].insert(j, 0);
                    cols[j].insert(i, 0);
                }
            }
        }

        for guard in guards.into_iter() {
            let (x, y) = (guard[0] as usize, guard[1] as usize);
            {
                let mut to_remove = vec![];
                let mut it = rows[x].range((y + 1)..);
                while let Some((&key, &value)) = it.next() {
                    if value == 0 {
                        to_remove.push(key);
                        cols[key].remove(&x);
                    } else {
                        break;
                    }
                }

                let mut it = rows[x].range(..y);
                while let Some((&key, &value)) = it.next_back() {
                    if value == 0 {
                        to_remove.push(key);
                        cols[key].remove(&x);
                    } else {
                        break;
                    }
                }

                for remove in to_remove {
                    rows[x].remove(&remove);
                }
            }
            {
                let mut to_remove = vec![];
                let mut it = cols[y].range((x + 1)..);
                while let Some((&key, &value)) = it.next() {
                    if value == 0 {
                        to_remove.push(key);
                        rows[key].remove(&y);
                    } else {
                        break;
                    }
                }

                let mut it = cols[y].range(..x);
                while let Some((&key, &value)) = it.next_back() {
                    if value == 0 {
                        to_remove.push(key);
                        rows[key].remove(&y);
                    } else {
                        break;
                    }
                }

                for remove in to_remove {
                    cols[y].remove(&remove);
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            for (k, t) in rows[i].iter() {
                if t == &0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
