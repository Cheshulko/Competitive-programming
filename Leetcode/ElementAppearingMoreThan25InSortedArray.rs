// https://leetcode.com/problems/element-appearing-more-than-25-in-sorted-array

struct Solution {}

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = (arr.len() as f64 / 4.).ceil() as usize;
        while fast != arr.len() {
            if arr[slow] == arr[fast] {
                break;
            }
            slow += 1;
            fast += 1;
        }
        arr[slow]
    }
}
