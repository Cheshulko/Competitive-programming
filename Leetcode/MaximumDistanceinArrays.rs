// https://leetcode.com/problems/maximum-distance-in-arrays

struct Solution {}

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let n = arrays.len();
        let mut ans = i32::MIN;

        let mut b = vec![];

        for i in 0..n {
            for &x in arrays[i].iter() {
                b.push((x, i));
            }
        }

        b.sort_unstable();
        for i in 0..b.len() {
            if b[i].1 != b.last().unwrap().1 {
                ans = ans.max((b[i].0 - b.last().unwrap().0).abs());
                break;
            }
        }
        for i in (0..b.len()).rev() {
            if b[i].1 != b[0].1 {
                ans = ans.max((b[i].0 - b[0].0).abs());
                break;
            }
        }

        ans
    }
}
