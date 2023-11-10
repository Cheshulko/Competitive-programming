// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs

struct Solution {}

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        const MX: i32 = 100_000;
        let mut m = vec![vec![]; 2 * MX as usize + 1];
        let mut cnt: Vec<usize> = vec![0; 2 * MX as usize + 1];
        let mut used = vec![false; 2 * MX as usize + 1];
        let mut ans = vec![];
        for x in adjacent_pairs.iter() {
            let x1 = (x[0] + MX) as usize;
            let x2 = (x[1] + MX) as usize;
            m[x1].push(x2);
            m[x2].push(x1);
            cnt[x1] += 1;
            cnt[x2] += 1;
        }
        let mut cur = cnt.iter().position(|x| x == &1).unwrap();
        loop {
            ans.push(cur as i32 - MX);
            used[cur] = true;
            if let Some(next) = m[cur].get(0) {
                if !used[*next] {
                    cur = *next;
                    continue;
                }
            }
            if let Some(next) = m[cur].get(1) {
                if !used[*next] {
                    cur = *next;
                    continue;
                }
            }
            break;
        }
        ans
    }
}
