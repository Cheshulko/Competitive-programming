// https://leetcode.com/problems/evaluate-reverse-polish-notation

struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let last_2 = |v: &mut Vec<i32>, op: &(dyn Fn(i32, i32) -> i32)| -> i32 {
            let s = v.pop().unwrap();
            let f = v.pop().unwrap();
            op(f, s)
        };
        *tokens
            .into_iter()
            .fold(vec![], |mut v, token| {
                let res = match token.as_str() {
                    "+" => last_2(&mut v, &std::ops::Add::add),
                    "-" => last_2(&mut v, &std::ops::Sub::sub),
                    "*" => last_2(&mut v, &std::ops::Mul::mul),
                    "/" => last_2(&mut v, &std::ops::Div::div),
                    /* number */
                    _ => token.parse::<i32>().unwrap(),
                };
                v.push(res);
                v
            })
            .last()
            .unwrap()
    }
}
