// https://leetcode.com/problems/find-the-winner-of-an-array-game

struct Solution {}

impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        let n = arr.len();
        let mut cur_ind = 0;
        let mut ind = 0;
        let mut streak = 0;
        loop {
            while ind < n && arr[cur_ind] >= arr[ind] {
                streak += 1;
                ind += 1;
            }
            if streak - 1 >= k {
                return arr[cur_ind];
            } else {
                if ind < n {
                    cur_ind = ind;
                    streak = 1;
                } else {
                    break;
                }
            }
        }
        *arr.iter().max().unwrap()
    }
}
