// https://leetcode.com/problems/find-all-people-with-secret

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        fn dijkstra(graph: &Vec<Vec<(usize, i32)>>, start: usize) -> Vec<Option<(usize, i32)>> {
            let mut ans = vec![None; graph.len()];
            let mut prio = BinaryHeap::new();

            for (new, time) in &graph[start] {
                match ans[*new] {
                    Some((_, new_time)) if new_time <= *time => {}
                    _ => {
                        ans[*new] = Some((start, *time));
                        prio.push(Reverse((*time, new, start)));
                    }
                }
            }

            while let Some(Reverse((secret_time, new, prev))) = prio.pop() {
                match ans[*new] {
                    Some((p, d)) if p == prev && d == secret_time => {}
                    _ => continue,
                }

                for (next, time) in &graph[*new] {
                    match ans[*next] {
                        Some((_, next_secret_time))
                            if secret_time > *time || next_secret_time <= *time => {}
                        None if secret_time > *time => {}
                        _ => {
                            ans[*next] = Some((*new, *time));
                            prio.push(Reverse((*time, next, *new)));
                        }
                    }
                }
            }

            ans
        }

        let (n, first_person) = (n as usize, first_person as usize);

        let mut adj = vec![vec![]; n + 1];
        adj[0].push((first_person, 0));
        adj[first_person].push((0, 0));
        for meeting in meetings.into_iter() {
            adj[meeting[0] as usize].push((meeting[1] as usize, meeting[2]));
            adj[meeting[1] as usize].push((meeting[0] as usize, meeting[2]));
        }

        dijkstra(&adj, 0)
            .into_iter()
            .enumerate()
            .filter_map(|x| x.1.is_some().then_some(x.0 as i32))
            .collect()
    }
}
