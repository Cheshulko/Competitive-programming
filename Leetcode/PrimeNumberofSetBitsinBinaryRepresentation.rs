// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation

struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
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

        let primes = eratosthenes(40);

        (left..=right)
            .filter(|&n| primes[n.count_ones() as usize])
            .count() as i32
    }
}
