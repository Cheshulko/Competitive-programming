// https://leetcode.com/problems/maximum-swap

struct Solution {}

impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut digits_pos = vec![vec![]; 10];
        let mut pos = vec![];
        let mut d = 0;
        while num > 0 {
            digits_pos[(num % 10) as usize].push(d);
            pos.push((num % 10) as usize);
            num /= 10;
            d += 1;
        }

        'out: for p in (0..pos.len()).rev() {
            for p2 in ((pos[p] + 1)..=9).rev() {
                if let Some(&first) = digits_pos[p2].first() {
                    if first < p {
                        pos.swap(p, first);
                        break 'out;
                    }
                }
            }
        }

        let num = pos
            .into_iter()
            .fold((0, 1), |(num, p), x| (num + x * p, p * 10))
            .0;
        num as i32
    }
}
