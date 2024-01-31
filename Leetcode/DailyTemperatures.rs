// https://leetcode.com/problems/daily-temperatures

struct Solution {}

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut temp_indexes = vec![vec![]; 102];

        let mut ans = vec![];
        for (ind, t) in temperatures.iter().enumerate().rev() {
            temp_indexes[*t as usize].push(ind);

            let t_next = *t as usize + 1;
            let indexes_next_gr = *temp_indexes[t_next..]
                .iter()
                .filter_map(|x| x.last())
                .min()
                .unwrap_or(&ind);
            ans.push((indexes_next_gr - ind) as i32);
        }

        ans.reverse();
        ans
    }
}
