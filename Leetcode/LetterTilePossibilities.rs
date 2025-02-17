// https://leetcode.com/problems/letter-tile-possibilities

use std::collections::{BTreeMap, HashSet};

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let tiles = tiles.chars().collect::<Vec<_>>();
        let tiles = tiles
            .into_iter()
            .scan(BTreeMap::new(), |m, x| {
                if let Some(&v) = m.get(&x) {
                    return Some(v);
                } else {
                    let v = m.len() + 1;
                    m.insert(x, v);

                    return Some(v);
                }
            })
            .collect::<Vec<_>>();

        fn go(cur: usize, tiles: &Vec<usize>, used: &mut Vec<bool>, have: &mut HashSet<usize>) {
            if used.iter().all(|x| *x) {
                have.insert(cur);

                return;
            }

            for i in 0..tiles.len() {
                if !used[i] {
                    used[i] = true;
                    go(cur * 10 + tiles[i], tiles, used, have);
                    used[i] = false;
                }
            }
        }

        let n = tiles.len();
        let mut have = HashSet::new();

        for mask in 1..(1 << n) {
            let tiles = tiles
                .iter()
                .enumerate()
                .filter_map(|(i, x)| (mask & (1 << i) > 0).then_some(*x))
                .collect::<Vec<_>>();

            let mut used = vec![false; tiles.len()];
            go(0, &tiles, &mut used, &mut have);
        }

        have.len() as i32
    }
}
