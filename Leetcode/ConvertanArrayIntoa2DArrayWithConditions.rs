// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions

struct Solution {}

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = vec![0; nums.len() + 1];
        let mut cnt = nums.len();

        for num in nums.iter() {
            arr[*num as usize] += 1;
        }

        let mut ans = vec![];
        while cnt > 0 {
            let mut cur = vec![];

            for i in 1..=nums.len() {
                if arr[i] > 0 {
                    cur.push(i as i32);
                    arr[i] -= 1;
                    cnt -= 1;
                }
            }

            ans.push(cur);
        }

        ans
    }
}
