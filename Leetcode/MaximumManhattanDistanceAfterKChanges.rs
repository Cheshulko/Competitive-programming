// https://leetcode.com/problems/maximum-manhattan-distance-after-k-changes

struct Solution {}

impl Solution {
    pub fn max_distance(s: String, k: i32) -> i32 {
        let n = s.len();
        let mut cnt: [i32; 2] = [0; 2];
        let mut d: [i32; 2] = [0; 2];
        let mut dm = vec![];
        let mut cm = vec![];
        for c in s.chars() {
            match c {
                'N' => {
                    cnt[0] += 1;
                    d[0] += 1;
                }
                'S' => {
                    cnt[0] += 1;
                    d[0] -= 1;
                }
                'E' => {
                    cnt[1] += 1;
                    d[1] += 1;
                }
                'W' => {
                    cnt[1] += 1;
                    d[1] -= 1;
                }
                _ => unreachable!(),
            }
            dm.push((d[0].abs(), d[1].abs()));
            cm.push((cnt[0], cnt[1]));
        }

        let mut ans = 0;
        for i in 0..n {
            let mut x = dm[i].0 + dm[i].1;
            let d =
                (cm[i].0.abs_diff(dm[i].0) + cm[i].1.abs_diff(dm[i].1)).min(2 * k as u32) as i32;
            x += d;
            ans = ans.max(x);
        }

        ans
    }
}
