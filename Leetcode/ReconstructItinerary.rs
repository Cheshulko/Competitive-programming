// https://leetcode.com/problems/reconstruct-itinerary

use std::collections::{BTreeMap, BTreeSet};

struct Solution {}

impl Solution {
    fn dfs(
        cur: usize,
        mtx: &Vec<Vec<usize>>,
        target_cnt: usize,
        used: &mut Vec<Vec<usize>>,
        path: &mut Vec<usize>,
    ) -> bool {
        if target_cnt + 1 == path.len() {
            return true;
        }
        for (to, to_cnt) in mtx[cur].iter().enumerate() {
            if used[cur][to] < *to_cnt {
                used[cur][to] += 1;
                path.push(to);

                if Solution::dfs(to, mtx, target_cnt, used, path) {
                    return true;
                }

                used[cur][to] -= 1;
                path.pop();
            }
        }

        return false;
    }

    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut unique = BTreeSet::<String>::new();

        for route in &tickets {
            let start = route[0].clone();
            let end = route[1].clone();

            unique.insert(start);
            unique.insert(end);
        }

        let n = unique.len();

        let mut point_to_ind = BTreeMap::<String, usize>::new();
        let mut ind_to_point = vec![];

        for point in unique.into_iter() {
            point_to_ind.insert(point.clone(), ind_to_point.len());
            ind_to_point.push(point);
        }

        let mut mtx = vec![vec![0; n]; n];

        for route in &tickets {
            let start = point_to_ind.get(&route[0]).unwrap();
            let end = point_to_ind.get(&route[1]).unwrap();

            mtx[*start][*end] += 1;
        }

        let mut path = vec![];
        let mut used = vec![vec![0; n]; n];

        let start = *point_to_ind.get("JFK".into()).unwrap();
        path.push(start);
        let _ = Solution::dfs(start, &mtx, tickets.len(), &mut used, &mut path);

        path.iter_mut()
            .map(|p| ind_to_point[*p].clone())
            .collect::<Vec<_>>()
    }
}
