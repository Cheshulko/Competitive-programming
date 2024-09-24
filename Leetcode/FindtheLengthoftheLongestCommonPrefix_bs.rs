// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr1 = arr1
            .into_iter()
            .map(|x| {
                let d = x.to_string().len();
                format!("{x}{y}", y = "$".repeat(9 - d))
            })
            .collect::<Vec<_>>();
        arr1.sort_unstable();

        arr2.into_iter()
            .map(|x| {
                let x = format!("{x}");

                let cnt = |ind: usize| {
                    arr1[ind]
                        .chars()
                        .zip(x.chars())
                        .take_while(|(a, b)| a == b)
                        .count()
                };

                let ind = arr1.partition_point(|y| y < &x);
                let mut c = 0;
                if ind != arr1.len() {
                    c = c.max(cnt(ind))
                }
                if ind != 0 {
                    c = c.max(cnt(ind - 1));
                }
                c
            })
            .max()
            .unwrap_or(0) as i32
    }
}
