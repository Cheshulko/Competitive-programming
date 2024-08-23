// https://leetcode.com/problems/fraction-addition-and-subtraction

mod math {
    pub fn gcd(a: i32, b: i32) -> i32 {
        let mut a = a.abs();
        let mut b = b.abs();

        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }

        while a != 0 {
            if a < b {
                std::mem::swap(&mut a, &mut b);
            }
            a %= b;
        }
        b
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        a / gcd(a, b) * b
    }
}

struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl TryFrom<&str> for Fraction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split('/');

        let numerator = split.next().ok_or(())?.parse::<i32>().map_err(|_| ())?;
        let denominator = split.next().ok_or(())?.parse::<i32>().map_err(|_| ())?;

        Ok(Fraction {
            numerator,
            denominator,
        })
    }
}

impl std::ops::Add for Fraction {
    type Output = Fraction;

    fn add(self, rhs: Self) -> Self::Output {
        let denominator = math::lcm(self.denominator, rhs.denominator);
        let numerator_left = self.numerator * (denominator / self.denominator);
        let numerator_right = rhs.numerator * (denominator / rhs.denominator);
        let numerator = numerator_left + numerator_right;
        let common = math::gcd(denominator, numerator);

        Fraction {
            numerator: numerator / common,
            denominator: denominator / common,
        }
    }
}

impl std::ops::Sub for Fraction {
    type Output = Fraction;

    fn sub(self, rhs: Self) -> Self::Output {
        let denominator = math::lcm(self.denominator, rhs.denominator);
        let numerator_left = self.numerator * (denominator / self.denominator);
        let numerator_right = rhs.numerator * (denominator / rhs.denominator);
        let numerator = numerator_left - numerator_right;
        let common = math::gcd(denominator, numerator);

        Fraction {
            numerator: numerator / common,
            denominator: denominator / common,
        }
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

struct Solution {}

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let expression = format!(
            "{leading}{expression}+",
            leading = ["+", ""][expression.starts_with("-") as usize]
        );

        let result = expression
            .match_indices(&['+', '-'])
            .skip(1)
            .fold((0, Fraction::default()), |(i, cur), (j, _)| {
                let (op, v) = (&expression[i..(i + 1)], &expression[(i + 1)..j]);
                if let Ok(fraction) = Fraction::try_from(v) {
                    let x = match op {
                        "+" => cur + fraction,
                        "-" => cur - fraction,
                        _ => unreachable!(),
                    };
                    (j, x)
                } else {
                    (j, cur)
                }
            })
            .1;

        format!("{}", result)
    }
}
