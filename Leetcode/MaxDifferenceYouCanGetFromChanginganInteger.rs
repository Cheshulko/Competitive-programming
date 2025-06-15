// https://leetcode.com/problems/max-difference-you-can-get-from-changing-an-integer

struct Solution {}

impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut nums = vec![];

        for from in 0..10 {
            'out: for to in 0..10 {
                let mut nnum = 0;
                let mut num2 = num;
                let mut p = 1;

                while num2 > 0 {
                    if num2 < 10 && num2 == from && to == 0 {
                        continue 'out;
                    }

                    let d = num2 % 10;
                    if from == d {
                        nnum += to * p;
                    } else {
                        nnum += d * p;
                    }

                    num2 /= 10;
                    p *= 10;
                }

                nums.push(nnum);
            }
        }

        nums.sort_unstable();

        *nums.iter().max().unwrap() - *nums.iter().min().unwrap()
    }
}
