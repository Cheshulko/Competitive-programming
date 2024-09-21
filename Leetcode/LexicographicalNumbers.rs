// https://leetcode.com/problems/lexicographical-numbers

struct Solution {}

impl Solution {
    fn go(cur: i32, n: i32, ans: &mut Vec<i32>) {
        for i in 0..10 {
            let x = cur * 10 + i;
            if x != cur && x <= n {
                ans.push(x);
                Solution::go(x, n, ans);
            }
        }
    }

    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        Solution::go(0, n, &mut ans);

        ans
    }
}
