// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box

struct Solution {}

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes
            .into_bytes()
            .into_iter()
            .map(|b| b == b'1')
            .collect::<Vec<_>>();

        let n = boxes.len();

        let mut ans = vec![0; n];
        {
            let mut sum = 0;
            let mut cnt = 0;
            for i in 0..n {
                ans[i] += i * cnt - sum;
                if boxes[i] {
                    cnt += 1;
                    sum += i;
                }
            }
        }
        {
            let mut sum = 0;
            let mut cnt = 0;
            for i in (0..n).rev() {
                ans[i] += sum - i * cnt;
                if boxes[i] {
                    cnt += 1;
                    sum += i;
                }
            }
        }

        ans.into_iter().map(|x| x as i32).collect()
    }
}
