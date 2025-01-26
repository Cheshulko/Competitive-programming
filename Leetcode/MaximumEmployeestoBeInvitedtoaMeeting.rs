// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting

struct Solution {}

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();

        let favorite = favorite.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let mut favorite_rev = vec![vec![]; n];
        for i in 0..n {
            favorite_rev[favorite[i]].push(i);
        }

        let mut pairs = vec![];
        for i in 0..n {
            let to = favorite[i];
            if favorite[to] == i {
                pairs.push((i, to));
            }
        }

        let mut used = vec![false; n];

        fn dfs_straight(cur: usize, favorite_rev: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> usize {
            used[cur] = true;

            let mut best = 1;
            for &from in favorite_rev[cur].iter() {
                if !used[from] {
                    best = best.max(1 + dfs_straight(from, favorite_rev, used));
                }
            }

            best
        }

        let mut sum_straights = 0;
        for (i, to) in pairs {
            if used[i] {
                assert!(used[to]);
                continue;
            }
            used[i] = true;
            used[to] = true;
            let len = dfs_straight(i, &favorite_rev, &mut used)
                + dfs_straight(to, &favorite_rev, &mut used);
            sum_straights += len;
        }

        fn dfs_cycle(
            cur: usize,
            dep: usize,
            favorite: &Vec<usize>,
            depths: &mut Vec<usize>,
            used: &mut Vec<bool>,
        ) -> usize {
            depths[cur] = dep;

            let to = favorite[cur];
            if used[to] {
                used[cur] = true;

                return 0;
            }

            let res = if depths[to] == 0 {
                dfs_cycle(to, dep + 1, favorite, depths, used)
            } else {
                depths[cur] - depths[to] + 1
            };

            used[cur] = true;
            res
        }

        let mut best_cycle = 0;

        let mut depths = vec![0; n];
        for i in 0..n {
            if used[i] {
                continue;
            }

            best_cycle = best_cycle.max(dfs_cycle(i, 1, &favorite, &mut depths, &mut used));
        }

        best_cycle.max(sum_straights) as i32
    }
}
