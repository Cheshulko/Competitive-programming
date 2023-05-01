// https://leetcode.com/problems/average-salary-excluding-the-minimum-and-maximum-salary

struct Solution {}

impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mn = salary.iter().min().unwrap();
        let mx = salary.iter().max().unwrap();
        let f = salary
            .iter()
            .filter(|x| x != &mn && x != &mx)
            .map(|x| *x as f64)
            .collect::<Vec<_>>();
        let len = f.len() as f64;
        f.iter().sum::<f64>() / len
    }
}
