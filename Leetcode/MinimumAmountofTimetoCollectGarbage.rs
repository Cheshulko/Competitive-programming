// https://leetcode.com/problems/minimum-amount-of-time-to-collect-garbage

struct Solution {}

impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, travel: Vec<i32>) -> i32 {
        let n = travel.len();
        let mut pref = vec![0; n + 2];

        for (i, house) in travel.into_iter().enumerate() {
            pref[i + 1] = pref[i] + house;
        }

        ['M', 'P', 'G'].into_iter().fold(0, |sum, t| {
            sum + garbage
                .iter()
                .enumerate()
                .fold((0, 0), |(mut sum, mut prev_stop), (ind, house)| {
                    let count = house.chars().filter(|c| c == t).count() as i32;
                    if count > 0 {
                        sum += pref[ind] - pref[prev_stop];
                        sum += count;
                        prev_stop = ind;
                    }
                    (sum, prev_stop)
                })
                .0
        })
    }
}
