// https://leetcode.com/problems/shifting-letters-ii

struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes().into_iter().collect::<Vec<_>>();
        let n = s.len();

        let mut absolute_shift = vec![0; n + 1];
        for shift in shifts {
            let (start, end, dir) = (shift[0] as usize, shift[1] as usize, shift[2]);

            if dir == 1 {
                absolute_shift[start] += 1;
                absolute_shift[end + 1] -= 1;
            } else {
                absolute_shift[start] -= 1;
                absolute_shift[end + 1] += 1;
            }
        }
        for i in 0..n {
            absolute_shift[i + 1] += absolute_shift[i];
        }

        for i in 0..n {
            let shift = (26 + absolute_shift[i] % 26) % 26;
            let shift = shift as u8;

            s[i] = b'a' + (s[i] - b'a' + shift) % 26;
        }

        String::from_utf8(s).unwrap()
    }
}
