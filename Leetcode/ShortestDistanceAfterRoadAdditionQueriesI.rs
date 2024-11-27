// https://leetcode.com/problems/shortest-distance-after-road-addition-queries-i

struct Solution {}

impl Solution {
    pub fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut d = (0..n).rev().collect::<Vec<_>>();

        let mut ans = vec![];

        let mut di = vec![vec![]; n];
        for i in 0..n - 1 {
            di[i].push(i + 1);
        }

        for q in queries.into_iter() {
            let u = q[0] as usize;
            let v = q[1] as usize;

            di[u].push(v);
            for _ in 0..2 {
                for i in (0..n - 1).rev() {
                    for &dii in di[i].iter() {
                        d[i] = d[i].min(d[dii] + 1);
                    }
                }
            }

            ans.push(d[0] as i32);
        }

        ans
    }
}
