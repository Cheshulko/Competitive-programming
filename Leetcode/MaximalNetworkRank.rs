// https://leetcode.com/problems/maximal-network-rank

struct Solution {}

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut mtx = vec![vec![false; n]; n];
        let mut cnt = vec![0; n];

        for road in &roads {
            let x = road[0] as usize;
            let y = road[1] as usize;
            mtx[x][y] = true;
            mtx[y][x] = true;
            cnt[x] += 1;
            cnt[y] += 1;
        }

        let mut ans = i32::MIN;

        for i in 0..n {
            for j in (i + 1)..n {
                ans = ans.max(cnt[i] + cnt[j] - (mtx[i][j] as i32));
            }
        }

        ans
    }
}
