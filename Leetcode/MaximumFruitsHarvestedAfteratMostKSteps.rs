// https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps

struct Solution {}

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let start_pos = start_pos as usize;
        let k = k as usize;

        let mut fruits = fruits
            .into_iter()
            .map(|fruit| (fruit[0] as usize, fruit[1]))
            .collect::<Vec<_>>();

        if fruits.iter().find(|(pos, _)| *pos == start_pos).is_none() {
            fruits.push((start_pos, 0));
            fruits.sort_unstable();
        }

        let start_pos_ind = fruits
            .iter()
            .position(|&(pos, _)| pos == start_pos)
            .unwrap();

        let mut pref = vec![0];
        for (i, &(_, cnt)) in fruits.iter().enumerate() {
            pref.push(pref[i] + cnt);
        }

        let mut ans = 0;
        for (cur_ind, &(pos1, _)) in fruits.iter().enumerate() {
            let dif = start_pos.abs_diff(pos1);
            if dif > k {
                continue;
            }

            let mut cur = 0;
            if pos1 < start_pos {
                cur += pref[start_pos_ind + 1] - pref[cur_ind];

                if k > 2 * dif {
                    let dif = k - 2 * dif;
                    let ind2 = fruits.partition_point(|&(pos, _)| pos <= start_pos + dif);
                    cur += pref[ind2] - pref[start_pos_ind + 1];
                }
            } else {
                cur += pref[cur_ind + 1] - pref[start_pos_ind];

                if k > 2 * dif {
                    let dif = (k - 2 * dif).min(start_pos);
                    let ind2 = fruits.partition_point(|&(pos, _)| pos < start_pos - dif);
                    cur += pref[start_pos_ind] - pref[ind2];
                }
            }

            ans = ans.max(cur);
        }

        ans
    }
}
