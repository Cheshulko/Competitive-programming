// https://leetcode.com/problems/compare-version-numbers

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1 = version1.split('.').collect::<Vec<_>>();
        let v2 = version2.split('.').collect::<Vec<_>>();

        for i in 0..v1.len().max(v2.len()) {
            let a = v1.get(i).unwrap_or(&"0").parse::<i32>().unwrap();
            let b = v2.get(i).unwrap_or(&"0").parse::<i32>().unwrap();

            match a.cmp(&b) {
                Ordering::Equal => continue,
                Ordering::Less => return -1,
                Ordering::Greater => return 1,
            };
        }

        return 0;
    }
}
