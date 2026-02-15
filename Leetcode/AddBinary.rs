// https://leetcode.com/problems/add-binary

struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        use std::iter::repeat;

        let n = a.len().max(b.len());

        let (carry, mut sum) = a
            .into_bytes()
            .into_iter()
            .rev()
            .chain(repeat(b'0'))
            .zip(b.into_bytes().into_iter().rev().chain(repeat(b'0')))
            .take(n)
            .fold((0, Vec::new()), |(mut carry, mut sum), (x, y)| {
                let x = x - b'0';
                let y = y - b'0';

                sum.push((x ^ y ^ carry) + b'0');
                carry = x & y | (carry & (x ^ y));

                (carry, sum)
            });

        if carry == 1 {
            sum.push(b'1');
        }
        sum.reverse();

        String::from_utf8(sum).unwrap()
    }
}
