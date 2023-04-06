/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let ans = code
        .chars()
        .all(|c| c.is_ascii_digit() || c.is_whitespace())
        .then(|| {
            let filtred = code
                .chars()
                .rev()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>();
            if filtred.len() < 2 {
                return false;
            } else {
                let x = filtred
                    .iter()
                    .enumerate()
                    .map(|every_sec| {
                        let u = every_sec.1.to_digit(10).unwrap();

                        if every_sec.0 % 2 == 0 {
                            u
                        } else {
                            if u > 4 {
                                2 * u - 9
                            } else {
                                2 * u
                            }
                        }
                    })
                    .collect::<Vec<u32>>()
                    .iter()
                    .fold(0, |s, v| s + *v);
                x % 10 == 0
            }
        })
        .unwrap_or(false);

    return ans;

    /*
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, cnt), x| {
            x.to_digit(10)
                .map(|num| if cnt % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, cnt + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    */
}
