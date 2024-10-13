// https://leetcode.com/problems/smallest-range-covering-elements-from-k-lists

struct Solution {}

impl Solution {
    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut al = *nums.iter().map(|x| x.iter().min().unwrap()).min().unwrap();
        let mut ar = *nums.iter().map(|x| x.iter().max().unwrap()).max().unwrap();

        for (cur_i, num) in nums.iter().enumerate() {
            for &x in num.iter() {
                let l = x;
                let mut r = x;

                let mut can = true;

                for (op_i, num) in nums.iter().enumerate() {
                    if cur_i == op_i {
                        continue;
                    }

                    let rr = num.partition_point(|&p| p < x); // >=
                    if rr != num.len() {
                        r = r.max(num[rr]);
                    } else {
                        can = false;
                    }
                }

                if can {
                    let adx = ar - al;
                    let dx = r - l;

                    if adx > dx {
                        al = l;
                        ar = r;
                    } else if adx == dx && l < al {
                        al = l;
                        ar = r;
                    }
                }
            }
        }

        vec![al, ar]
    }
}
