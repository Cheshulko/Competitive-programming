// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks

struct Solution {}

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let k = k as usize;
        let blocks = blocks
            .into_bytes()
            .into_iter()
            .map(|b| (b == b'B') as usize)
            .collect::<Vec<_>>();

        let mut cur = blocks.iter().take(k).sum::<usize>();
        let mut ans = k - cur;

        for i in k..blocks.len() {
            cur -= blocks[i - k];
            cur += blocks[i];

            ans = ans.min(k - cur);
        }

        ans as i32
    }
}
