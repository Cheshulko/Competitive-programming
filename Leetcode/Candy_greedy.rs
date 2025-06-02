// https://leetcode.com/problems/candy/submissions

struct Solution {}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let ratings = ratings
            .into_iter()
            .map(|rating| rating as usize)
            .collect::<Vec<_>>();

        let ma = 1 + ratings.iter().max().copied().unwrap();
        let n = ratings.len();

        let mut inds = vec![vec![]; ma];
        for (ind, &rating) in ratings.iter().enumerate() {
            inds[rating].push(ind);
        }

        let mut ans = ratings
            .clone()
            .into_iter()
            .map(|x| x + 1)
            .collect::<Vec<_>>();

        for (rating, inds) in inds.into_iter().enumerate() {
            for ind in inds.into_iter() {
                let left = if ind > 0 && ratings[ind - 1] < rating {
                    ans[ind - 1] + 1
                } else {
                    1
                };

                let right = if ind + 1 < n && ratings[ind + 1] < rating {
                    ans[ind + 1] + 1
                } else {
                    1
                };

                ans[ind] = ans[ind].min(left.max(right));
            }
        }

        ans.into_iter().sum::<usize>() as i32
    }
}
