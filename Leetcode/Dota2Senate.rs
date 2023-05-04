// https://leetcode.com/problems/dota2-senate

struct Solution {}

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        // 0 - R, 1 - D
        let len = senate.len();
        let mut cnt: [usize; 2] = [0, 0];
        cnt[0] = senate.chars().filter(|c| c == &'R').count();
        cnt[1] = len - cnt[0];

        let mut v = senate
            .chars()
            .map(|c| match c {
                'R' => 0,
                'D' => 1,
                _ => panic!(),
            })
            .collect::<Vec<i32>>();

        let mut i = 0;
        let mut cur = v[i];
        let mut streak = 1;
        i += 1;
        while cnt[0] > 0 && cnt[1] > 0 {
            if v[i] == -1 {
                i = (i + 1) % len;
                continue;
            }
            if v[i] == cur {
                streak += 1;
            } else {
                if streak > 0 {
                    streak -= 1;
                    cnt[v[i] as usize] -= 1;
                    v[i] = -1;
                } else {
                    streak = 1;
                    cur = v[i];
                }
            }
            i = (i + 1) % len;
        }

        match (cnt[0] > 0, cnt[1] > 0) {
            (true, false) => "Radiant",
            (false, true) => "Dire",
            _ => panic!(),
        }
        .to_string()
    }
}
