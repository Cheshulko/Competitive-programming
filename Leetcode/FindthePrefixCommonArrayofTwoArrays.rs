// https://leetcode.com/problems/find-the-prefix-common-array-of-two-arrays

struct Solution {}

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut state = vec![0; 50 + 1];
        let mut cnt = 0;

        let mut ans = vec![];
        for (a, b) in a.into_iter().zip(b.into_iter()) {
            let (a, b) = (a as usize, b as usize);

            state[a] += 1;
            if state[a] == 2 {
                cnt += 1;
            }
            state[b] += 1;
            if state[b] == 2 {
                cnt += 1;
            }

            ans.push(cnt);
        }

        ans
    }
}
