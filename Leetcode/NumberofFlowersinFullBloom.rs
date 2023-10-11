// https://leetcode.com/problems/number-of-flowers-in-full-bloom

struct Solution {}

impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut start = flowers.iter().map(|v| v[0]).collect::<Vec<_>>();
        let mut end = flowers.iter().map(|v| v[1]).collect::<Vec<_>>();
        start.sort();
        end.sort();

        let mut ans = vec![];
        for p in people.into_iter() {
            let started = start.partition_point(|x| x <= &p);
            let ended = end.partition_point(|x| x < &p);
            ans.push((started - ended) as i32);
        }

        ans
    }
}
