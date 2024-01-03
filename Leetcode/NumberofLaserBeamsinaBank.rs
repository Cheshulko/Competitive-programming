// https://leetcode.com/problems/number-of-laser-beams-in-a-bank

struct Solution {}

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.into_iter()
            .filter(|x| !x.chars().all(|c| c == '0'))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.chars().filter(|x| x == &'1').count())
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, |ans, cur| ans + cur[0] * cur[1]) as i32
    }
}
