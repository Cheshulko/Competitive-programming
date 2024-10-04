// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill

struct Solution {}

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let skills = skill.into_iter().map(|x| x as usize).collect::<Vec<_>>();
        let n = skills.len();

        let s = skills.iter().sum::<usize>();
        if s % (n / 2) != 0 {
            return -1;
        }

        let mut cnt = vec![0; 1001];
        for &skill in skills.iter() {
            cnt[skill] += 1;
        }

        let per = s / (n / 2);

        let mut ans = 0;
        for skill in skills.into_iter() {
            if cnt[skill] > 0 {
                if skill > per {
                    return -1;
                }
                let need = per - skill;
                if cnt[need] == 0 {
                    return -1;
                }
                ans += skill * need;
                cnt[skill] -= 1;
                cnt[need] -= 1;
            }
        }

        ans as i64
    }
}
