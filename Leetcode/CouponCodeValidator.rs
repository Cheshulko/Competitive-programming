// https://leetcode.com/problems/coupon-code-validator

struct Solution {}

impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        const T: &[&str] = &["electronics", "grocery", "pharmacy", "restaurant"];

        let mut x = code
            .into_iter()
            .zip(business_line.into_iter())
            .zip(is_active.into_iter())
            .filter_map(|((a, b), c)| {
                (c && T.contains(&b.as_str())
                    && a.chars()
                        .all(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_'))
                    && a.len() > 0)
                    .then_some((T.iter().position(|x| x == &b.as_str()), c, a))
            })
            .collect::<Vec<_>>();

        x.sort_unstable();

        x.into_iter().map(|(_, _, a)| a).collect()
    }
}
