// https://leetcode.com/problems/maximum-matching-of-players-with-trainers

struct Solution {}

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();

        let n = players.len();

        let mut i = 0;
        let mut cnt = 0;

        for t in trainers {
            if i == n {
                break;
            }

            if t >= players[i] {
                cnt += 1;
                i += 1;
            }
        }

        cnt
    }
}
