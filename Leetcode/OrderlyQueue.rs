// https://leetcode.com/problems/orderly-queue

struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k > 1 {
            let mut char_vec: Vec<char> = s.chars().collect();
            char_vec.sort();
            return char_vec.iter().collect::<String>();
        } else {
            let mut best = s.clone();
            let mut candidate = s.clone();
            let mut tries = s.len();
            while tries > 1 {
                let c = candidate.pop().unwrap();
                candidate = format!("{}{}", c.to_string(), candidate);
                tries -= 1;
                if candidate < best {
                    best = candidate.clone();
                }
            }
            return best;
        }
    }
}
