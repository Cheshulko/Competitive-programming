// https://leetcode.com/problems/binary-watch

struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let turned_on = turned_on as u32;

        let mut ans = vec![];
        for top_mask in 0..(1_u32 << 4) {
            for bottom_mask in 0..(1_u32 << 6) {
                if top_mask.count_ones() + bottom_mask.count_ones() != turned_on {
                    continue;
                }

                if top_mask >= 12 || bottom_mask >= 60 {
                    continue;
                }

                let h = format!("{top_mask:002}");
                let m = format!("{bottom_mask:002}");

                ans.push(format!(
                    "{}:{}",
                    if h == "00" {
                        "0"
                    } else {
                        h.trim_start_matches('0')
                    },
                    m
                ));
            }
        }

        ans
    }
}
