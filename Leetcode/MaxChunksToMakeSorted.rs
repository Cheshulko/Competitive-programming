// https://leetcode.com/problems/max-chunks-to-make-sorted

struct Solution {}

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut mi = usize::MAX;
        let mut ma = usize::MIN;
        let mut cnt = 0;
        let mut ans = 0;

        for i in 0..n {
            mi = mi.min(arr[i] as usize);
            ma = ma.max(arr[i] as usize);
            cnt += 1;

            if ma == i && mi == i + 1 - cnt {
                mi = usize::MAX;
                ma = usize::MIN;
                ans += 1;
                cnt = 0;
            }
        }

        ans as i32
    }
}
