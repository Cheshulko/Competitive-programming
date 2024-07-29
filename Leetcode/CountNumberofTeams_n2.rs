// https://leetcode.com/problems/count-number-of-teams

struct Solution {}

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        const MAX: usize = 100_000;

        let n = rating.len();
        let mut less = vec![0; MAX + 1];
        let mut grea = vec![0; MAX + 1];

        let mut ans = 0;
        for i in 0..n {
            for j in 0..i {
                if rating[j] < rating[i] {
                    ans += less[rating[j] as usize];

                    less[rating[i] as usize] += 1;
                }

                if rating[j] > rating[i] {
                    ans += grea[rating[j] as usize];

                    grea[rating[i] as usize] += 1;
                }
            }
        }

        ans
    }
}
