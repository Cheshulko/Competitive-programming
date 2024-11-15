// https://leetcode.com/problems/shortest-subarray-to-be-removed-to-make-array-sorted

struct Solution {}

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut head = vec![arr[0]];
        let mut tail = vec![arr[n - 1]];

        for i in 1..n {
            if arr[i - 1] <= arr[i] {
                head.push(arr[i]);
            } else {
                break;
            }
        }

        for i in (0..n - 1).rev() {
            if arr[i + 1] >= arr[i] {
                tail.push(arr[i]);
            } else {
                break;
            }
        }
        tail.reverse();

        let mut ans = n - head.len().max(tail.len());
        for i in 0..head.len() {
            let p = tail.partition_point(|&x| x < head[i]);
            ans = ans.min(n - (i + 1 + tail.len() - p));
        }

        ans as i32
    }
}
