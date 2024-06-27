// https://leetcode.com/problems/find-center-of-star-graph

struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.iter().map(|v| v[0].max(v[1])).max().unwrap() as usize;
        let mut cnt = vec![0; n + 1];
        for edge in edges.into_iter() {
            cnt[edge[0] as usize] += 1;
            cnt[edge[1] as usize] += 1;
        }

        cnt.into_iter()
            .enumerate()
            .filter_map(|(i, x)| (x > 1).then_some(i))
            .next()
            .unwrap() as i32
    }
}
