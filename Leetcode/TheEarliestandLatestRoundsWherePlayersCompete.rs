// https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        fn solve(
            round: i32,
            n: usize,
            state: i32,
            f: i32,
            s: i32,
            dp_min: &mut HashMap<i32, i32>,
            dp_max: &mut HashMap<i32, i32>,
        ) {
            let mut l = 1 << 0;
            let mut r = 1 << (n - 1);

            let mut next_state = 0;
            let mut p = vec![];
            let mut met = false;
            while l <= r {
                while l < r && (state & l == 0) {
                    l <<= 1;
                }
                while l < r && (state & r == 0) {
                    r >>= 1;
                }

                if l == r {
                    if (state & l) > 0 {
                        next_state |= l;
                    }
                } else {
                    assert!(state & l > 0);
                    assert!(state & r > 0);

                    if l == f && r == s {
                        dp_min
                            .entry(state)
                            .and_modify(|r| *r = (*r).min(round))
                            .or_insert(round);
                        dp_max
                            .entry(state)
                            .and_modify(|r| *r = (*r).max(round))
                            .or_insert(round);
                        met = true;
                    } else {
                        if l == f || l == s {
                            next_state |= l;
                        } else if r == f || r == s {
                            next_state |= r;
                        } else {
                            p.push((l, r));
                        }
                    }
                }

                l <<= 1;
                r >>= 1;
            }

            if met {
                return;
            }

            if p.is_empty() {
                if dp_min.contains_key(&next_state) {
                    assert!(dp_max.contains_key(&next_state));
                } else {
                    solve(round + 1, n, next_state, f, s, dp_min, dp_max);
                }
                let next_min = *dp_min.get(&next_state).unwrap();
                let next_max = *dp_max.get(&next_state).unwrap();

                dp_min
                    .entry(state)
                    .and_modify(|r| *r = (*r).min(next_min))
                    .or_insert(next_min);
                dp_max
                    .entry(state)
                    .and_modify(|r| *r = (*r).max(next_max))
                    .or_insert(next_max);
            } else {
                for i in 0..(1 << p.len()) {
                    let mut next_state_2 = next_state;

                    for b in 0..p.len() {
                        if (i & (1 << b)) == 0 {
                            next_state_2 |= p[b].0;
                        } else {
                            next_state_2 |= p[b].1;
                        }
                    }

                    if dp_min.contains_key(&next_state_2) {
                        assert!(dp_max.contains_key(&next_state_2));
                    } else {
                        solve(round + 1, n, next_state_2, f, s, dp_min, dp_max);
                    }

                    let next_min = *dp_min.get(&next_state_2).unwrap();
                    let next_max = *dp_max.get(&next_state_2).unwrap();

                    dp_min
                        .entry(state)
                        .and_modify(|r| *r = (*r).min(next_min))
                        .or_insert(next_min);
                    dp_max
                        .entry(state)
                        .and_modify(|r| *r = (*r).max(next_max))
                        .or_insert(next_max);
                }
            }
        }

        let n = n as usize;
        let mut dp_min = HashMap::new();
        let mut dp_max = HashMap::new();

        let (first_player, second_player) = (
            first_player.min(second_player),
            first_player.max(second_player),
        );

        solve(
            1,
            n,
            (1 << n) - 1,
            1 << (first_player - 1),
            1 << (second_player - 1),
            &mut dp_min,
            &mut dp_max,
        );

        vec![
            *dp_min.get(&((1 << n) - 1)).unwrap(),
            *dp_max.get(&((1 << n) - 1)).unwrap(),
        ]
    }
}
