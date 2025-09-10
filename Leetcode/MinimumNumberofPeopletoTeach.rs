// https://leetcode.com/problems/minimum-number-of-people-to-teach

struct Solution {}

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let n = n as usize;

        let languages = languages
            .into_iter()
            .map(|v| v.into_iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>();

        let already_can = friendships
            .iter()
            .map(|f| {
                let a = f[0] as usize - 1;
                let b = f[1] as usize - 1;

                languages[a].intersection(&languages[b]).count() > 0
            })
            .collect::<Vec<_>>();

        let mut ans = i32::MAX;
        let mut taught = vec![vec![false; n as usize + 1]; languages.len()];

        for l in 1..=n {
            let mut cnt = 0;
            for row in taught.iter_mut() {
                row.fill(false);
            }

            for (i, f) in friendships.iter().enumerate() {
                let a = f[0] as usize - 1;
                let b = f[1] as usize - 1;

                if already_can[i] {
                    continue;
                }

                cnt += (!languages[a].contains(&(l as i32)) && !taught[a][l]) as i32;
                cnt += (!languages[b].contains(&(l as i32)) && !taught[b][l]) as i32;

                taught[a][l] = true;
                taught[b][l] = true;
            }

            ans = ans.min(cnt);
        }

        ans
    }
}
