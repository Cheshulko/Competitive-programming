pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split([' ', '-'])
        .filter(|s| !s.is_empty())
        .map(|s| {
            let f = s.chars().next().unwrap_or_default().to_ascii_uppercase();
            let mut t = String::from(f);
            t.push_str(&s[1..]);
            t
        })
        .fold(String::new(), |mut s, t| {
            if t.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .all(|c| c.is_ascii_uppercase())
            {
                s.push(t.chars().next().unwrap_or_default())
            } else {
                let x = t
                    .chars()
                    .filter(|c| c.is_alphabetic() && c.is_ascii_uppercase());
                s.extend(x);
            }

            s
        })
        .to_string()
}
