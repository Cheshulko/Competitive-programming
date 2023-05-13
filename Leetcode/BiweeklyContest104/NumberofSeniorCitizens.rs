// https://leetcode.com/contest/biweekly-contest-104/problems/number-of-senior-citizens/

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        details
            .iter()
            .map(|x| x.get(11..13).unwrap().parse::<i32>().unwrap())
            .filter(|x| x > &60)
            .count() as i32
    }
}
