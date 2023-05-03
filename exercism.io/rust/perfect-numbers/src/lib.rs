#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let sq = (num as f64).sqrt().floor() as i64;
    let num = num as i64;

    match num {
        1.. => Some(
            match 2 * num
                - (1..=sq)
                    .filter(|x| &num % x == 0)
                    .fold(0, |sum, value| sum + value + num / value)
                + if sq * sq == num { sq } else { 0 }
            {
                1.. => Classification::Deficient,
                0 => Classification::Perfect,
                _ => Classification::Abundant,
            },
        ),
        _ => None,
    }
}
