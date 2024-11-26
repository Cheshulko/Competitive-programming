// https://leetcode.com/problems/find-champion-ii

struct Solution {}

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut ins = vec![0; n];

        for edge in edges {
            ins[edge[1] as usize] += 1;
        }

        let mut ans = -1;
        for i in 0..n {
            if ins[i] == 0 {
                if ans == -1 {
                    ans = i as i32;
                } else {
                    return -1;
                }
            }
        }

        ans
    }
}
