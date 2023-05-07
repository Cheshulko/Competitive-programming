use std::iter::repeat;

pub fn encode(source: &str) -> String {
    let mut cur_op: Option<char> = None;
    let mut cnt = 0;
    let mut ans = String::new();

    let mut source = String::from(source);
    source.push('-');

    for c in source.chars() {
        if let Some(cur) = cur_op {
            if cur != c {
                if cnt > 1 {
                    ans.extend(format!("{}{}", cnt, cur).chars());
                } else {
                    ans.extend(format!("{}", cur).chars());
                }
                cur_op = Some(c);
                cnt = 1;
            } else {
                cnt += 1;
            }
        } else {
            cur_op = Some(c);
            cnt = 1;
        }
    }

    ans
}

pub fn decode(source: &str) -> String {
    let mut cnt = 0;
    let mut ans = String::new();

    for c in source.chars() {
        if let Some(d) = c.to_digit(10) {
            cnt = 10 * cnt + d as usize;
        } else {
            ans.extend(repeat(c).take(cnt.max(1)));
            cnt = 0;
        }
    }

    ans
}
