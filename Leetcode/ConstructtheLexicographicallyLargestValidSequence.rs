// https://leetcode.com/problems/construct-the-lexicographically-largest-valid-sequence

struct Solution {}

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;

        fn solve(p: usize, n: usize, used: &mut Vec<bool>, ans: &mut Vec<usize>) -> bool {
            if used.iter().all(|x| *x) {
                return true;
            }

            if p == 2 * (n - 1) + 1 {
                return false;
            }

            if ans[p] != 0 {
                return solve(p + 1, n, used, ans);
            }

            for i in (1..=n).rev() {
                let di = if i == 1 { 0 } else { i };

                if !used[i] && p + di < 2 * (n - 1) + 1 && ans[p + di] == 0 {
                    used[i] = true;
                    ans[p] = i;
                    ans[p + di] = i;

                    if solve(p + 1, n, used, ans) {
                        return true;
                    }

                    ans[p] = 0;
                    ans[p + di] = 0;
                    used[i] = false;
                }
            }

            false
        }

        let mut ans = vec![0; 2 * (n - 1) + 1];
        let mut used = vec![false; n + 1];
        used[0] = true;
        solve(0, n, &mut used, &mut ans);

        ans.into_iter().map(|x| x as i32).collect()
    }
}
