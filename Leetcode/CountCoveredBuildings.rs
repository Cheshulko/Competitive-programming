// https://leetcode.com/problems/count-covered-buildings

struct Solution {}

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        use std::collections::{BTreeMap, BTreeSet};

        let buildings = buildings
            .into_iter()
            .map(|building| {
                let &[x, y] = building.as_slice() else {
                    panic!()
                };
                (x, y)
            })
            .collect::<Vec<_>>();

        let mut xs = BTreeMap::<i32, BTreeSet<i32>>::new();
        let mut ys = BTreeMap::<i32, BTreeSet<i32>>::new();
        for (i, &(x, y)) in buildings.iter().enumerate() {
            xs.entry(x).or_default().insert(y);
            ys.entry(y).or_default().insert(x);
        }

        let check = |x: i32, y: i32| -> bool {
            let ok_by_x = xs
                .get(&x)
                .and_then(|ys| {
                    (*ys.first().unwrap() != y && *ys.last().unwrap() != y).then_some(true)
                })
                .is_some();

            let ok_by_y = ys
                .get(&y)
                .and_then(|xs| {
                    (*xs.first().unwrap() != x && *xs.last().unwrap() != x).then_some(true)
                })
                .is_some();

            ok_by_x && ok_by_y
        };

        let mut ans = 0;
        for (x, y) in buildings.into_iter() {
            if check(x, y) {
                ans += 1;
            }
        }

        ans
    }
}
