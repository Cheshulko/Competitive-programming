// https://leetcode.com/problems/parsing-a-boolean-expression

struct Solution {}

impl Solution {
    fn parse<'input>(mut e: &'input [u8]) -> (&'input [u8], Vec<bool>) {
        let mut values = vec![];

        while !e.is_empty() {
            match e[0] {
                b'&' => {
                    let (rest, inner_values) = Solution::parse(&e[2..]);
                    let inner_value = inner_values.into_iter().fold(true, |v, x| x && v);

                    e = rest;
                    values.push(inner_value);
                }
                b'|' => {
                    let (rest, inner_values) = Solution::parse(&e[2..]);
                    let inner_value = inner_values.into_iter().fold(false, |v, x| x || v);

                    e = rest;
                    values.push(inner_value);
                }
                b'!' => {
                    let (rest, inner_values) = Solution::parse(&e[2..]);
                    assert!(inner_values.len() == 1);

                    e = rest;
                    values.push(!inner_values[0]);
                }
                b't' => {
                    e = &e[1..];
                    values.push(true);
                }
                b'f' => {
                    e = &e[1..];
                    values.push(false);
                }
                b',' => {
                    e = &e[1..];
                }
                b')' => {
                    e = &e[1..];
                    break;
                }
                x => unreachable!("{}", x as char),
            }
        }

        return (e, values);
    }

    pub fn parse_bool_expr(expression: String) -> bool {
        let expression = expression.into_bytes();
        let (rest, result) = Solution::parse(&expression);
        assert!(rest.is_empty());
        assert!(result.len() == 1);

        result[0]
    }
}
