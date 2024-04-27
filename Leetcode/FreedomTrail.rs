// https://leetcode.com/problems/freedom-trail

struct Solution {}

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.into_bytes();
        let key = key.into_bytes();
        let ring_n = ring.len();
        let key_n = key.len();

        let char_pos = ring
            .into_iter()
            .enumerate()
            .fold(vec![vec![]; 26], |mut v, (i, c)| {
                v[(c - b'a') as usize].push(i);
                v
            });

        let mut dp = vec![vec![vec![]; 26]; key_n];

        for c in 0..26 {
            for &pos_c in &char_pos[c] {
                dp[0][c].push((pos_c, pos_c.min(ring_n - pos_c)));
            }
        }

        for i in 1..key_n {
            let need_c = (key[i] - b'a') as usize;
            let prev_c = (key[i - 1] - b'a') as usize;

            for &pos_can in &char_pos[need_c] {
                let mut dist_pos = usize::MAX;
                for &(prev_pos, prev_dist) in &dp[i - 1][prev_c] {
                    let dist = (pos_can + ring_n - prev_pos) % ring_n;
                    let dist = dist.min(ring_n - dist);

                    dist_pos = dist_pos.min(dist + prev_dist);
                }

                dp[i][need_c].push((pos_can, dist_pos));
            }
        }

        let mut ans = usize::MAX;
        for &(_, dist) in &dp[key_n - 1][(key[key_n - 1] - b'a') as usize] {
            ans = ans.min(dist);
        }

        (ans + key_n) as i32
    }
}
