// https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions

struct Solution {}

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        let skill = skill.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let mana = mana.into_iter().map(|x| x as i64).collect::<Vec<_>>();

        let n = skill.len();

        let mut end_times = vec![0; n];
        let mut nend_times = vec![0; n];
        for potion in mana {
            nend_times[0] = end_times[0] + potion * skill[0];
            for i in 1..n {
                nend_times[i] = nend_times[i - 1] + potion * skill[i];
            }

            let mut delta = 0;
            for i in 0..n {
                delta = delta.max(end_times[i] - (nend_times[i] - potion * skill[i]));
            }

            for i in 0..n {
                nend_times[i] += delta;
            }

            std::mem::swap(&mut end_times, &mut nend_times);
        }

        end_times.into_iter().max().unwrap_or(0)
    }
}
