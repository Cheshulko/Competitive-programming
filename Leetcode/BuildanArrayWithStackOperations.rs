// https://leetcode.com/problems/build-an-array-with-stack-operations

struct Solution {}

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans = vec![];
        let mut cur = 1;

        for t in target.into_iter() {
            while t > cur {
                cur += 1;
                ans.push("Push".into());
                ans.push("Pop".into());
            }
            ans.push("Push".into());
            cur += 1;
        }

        ans
    }
}
