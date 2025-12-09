// https://leetcode.com/problems/count-special-triplets

struct Solution {}

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const M: usize = 1_000_000_000 + 7;

        let ma = 2 * nums.iter().max().copied().unwrap() as usize + 1;

        let mut counts: [Vec<usize>; 3] = std::array::from_fn(|_| vec![0; ma]);
        for num in nums.into_iter() {
            let num = num as usize;

            if num % 2 == 0 {
                counts[2][num] = (counts[2][num] + counts[1][num / 2]) % M;
            }

            counts[1][num] = (counts[1][num] + counts[0][num * 2]) % M;
            counts[0][num] += 1;
        }

        let mut ans = 0;
        for &cnt in counts[2].iter() {
            ans = (ans + cnt) % M;
        }

        ans as i32
    }
}
