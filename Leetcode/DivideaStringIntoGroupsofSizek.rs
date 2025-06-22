// https://leetcode.com/problems/divide-a-string-into-groups-of-size-k

struct Solution {}

impl Solution {
    pub fn divide_string(mut s: String, k: i32, fill: char) -> Vec<String> {
        let k = k as usize;

        s.extend(String::from(fill).repeat((k - s.len() % k) % k).chars());
        s.into_bytes()
            .chunks(k)
            .map(|ch| String::from_utf8(ch.to_owned()).unwrap())
            .collect()
    }
}
