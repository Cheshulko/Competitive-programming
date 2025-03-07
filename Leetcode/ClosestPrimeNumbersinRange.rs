// https://leetcode.com/problems/closest-prime-numbers-in-range

struct Solution {}

impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        fn eratosthenes(n: usize) -> Vec<bool> {
            let mut pr = vec![true; n + 1];
            pr[0] = false;
            pr[1] = false;
            for i in 2..=n {
                if pr[i] {
                    for j in (2 * i..=n).step_by(i) {
                        pr[j] = false;
                    }
                }
            }
            pr
        }

        let left = left as usize;
        let right = right as usize;

        let pr = eratosthenes(right + 1)
            .into_iter()
            .enumerate()
            .filter_map(|(i, p)| p.then_some(i))
            .collect::<Vec<_>>();

        let mut d = usize::MAX;
        let mut l = 0;
        let mut r = 0;

        let st = pr.partition_point(|&x| x < left);
        for i in st..pr.len() - 1 {
            let x = pr[i];
            let y = pr[i + 1];
            if y > right {
                break;
            }

            if y - x < d {
                l = x;
                r = y;
                d = y - x;
            }
        }

        if d == usize::MAX {
            vec![-1, -1]
        } else {
            vec![l as i32, r as i32]
        }
    }
}
