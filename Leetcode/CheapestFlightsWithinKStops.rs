// https://leetcode.com/problems/cheapest-flights-within-k-stops

use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let (n, src, dst, k) = (n as usize, src as usize, dst as usize, 1 + k as usize);

        let mut adj = vec![vec![]; n + 1];
        for flight in flights.into_iter() {
            adj[flight[0] as usize].push((flight[1] as usize, flight[2] as usize));
        }

        let mut dists = vec![usize::MAX; n + 1];
        dists[src] = 0;

        let mut q = VecDeque::<(usize, usize, usize)>::new();
        q.push_back((dists[src], src, 0));

        while let Some((dist, ind, steps)) = q.pop_front() {
            if steps == k {
                continue;
            }

            for (to, len) in &adj[ind] {
                if dists[*to] > *len + dist {
                    dists[*to] = *len + dist;
                    q.push_back((dists[*to], *to, steps + 1));
                }
            }
        }

        dists[dst] as i32
    }
}
