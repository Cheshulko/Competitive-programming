// https://leetcode.com/problems/minimum-remove-to-make-valid-parentheses

struct Solution {}

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        fn pass(s: impl Iterator<Item = u8>, open: u8, close: u8) -> Vec<u8> {
            s.fold((0, Vec::<u8>::new()), |(mut state, mut v), c| {
                if c == open {
                    state += 1;
                    v.push(c);
                } else if c == close {
                    if state > 0 {
                        state -= 1;
                        v.push(c);
                    }
                } else {
                    v.push(c);
                }

                (state, v)
            })
            .1
        }

        let s = s.into_bytes();
        let s = pass(s.into_iter(), b'(', b')');
        let mut s = pass(s.into_iter().rev(), b')', b'(');
        s.reverse();

        String::from_utf8(s).unwrap()
    }
}
