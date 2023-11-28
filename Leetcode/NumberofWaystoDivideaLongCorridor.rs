// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor

struct Solution {}

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const M: i64 = 1_000_000_000 + 7;
        let mut positions = vec![];
        let mut streak = 0;
        let mut ans = 1;

        let corridor = corridor.trim_matches('P');
        for c in corridor.chars() {
            if c == 'S' {
                streak += 1;
                positions.push(false);
                if streak == 2 {
                    positions.push(true);
                    streak = 0;
                }
            } else {
                if streak == 0 {
                    positions.push(true);
                }
            }
        }

        for can_set_div in positions.into_iter() {
            if can_set_div {
                streak += 1;
            } else if streak != 0 {
                ans = (ans * streak) % M;
                streak = 0;
            }
        }

        ((ans * streak) % M) as i32
    }
}
