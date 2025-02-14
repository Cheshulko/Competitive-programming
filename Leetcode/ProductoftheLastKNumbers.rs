// https://leetcode.com/problems/product-of-the-last-k-numbers

struct ProductOfNumbers {
    pref_prod: Vec<i64>,
    pref_zero: Vec<usize>,
}

impl ProductOfNumbers {
    const MOD: i64 = 1_000_000_000 + 7;

    fn new() -> Self {
        ProductOfNumbers {
            pref_prod: vec![1],
            pref_zero: vec![0],
        }
    }

    fn add(&mut self, num: i32) {
        let last = *self.pref_prod.last().unwrap();
        let last_zero = *self.pref_zero.last().unwrap();

        if num == 0 {
            self.pref_prod.push(last);
            self.pref_zero.push(last_zero + 1);
        } else {
            let next = (last * num as i64) % ProductOfNumbers::MOD;
            self.pref_prod.push(next);
            self.pref_zero.push(last_zero);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        let len = self.pref_prod.len();

        let pref = (self.pref_prod[len - 1]
            * self.mod_inverse(self.pref_prod[len - 1 - k], ProductOfNumbers::MOD))
            % ProductOfNumbers::MOD;
        let zero = self.pref_zero[len - 1] - self.pref_zero[len - 1 - k];

        if zero > 0 {
            0
        } else {
            pref as i32
        }
    }

    fn mod_inverse(&self, mut x: i64, m: i64) -> i64 {
        let mut res = 1;
        let mut exp = m - 2; // Fermat's little theorem

        // Binary exponentiation b ^ (m - 2)
        while exp != 0 {
            if exp % 2 == 1 {
                res = res * x % m;
            }
            x = x * x % m;
            exp /= 2;
        }
        res
    }
}
