pub fn check(candidate: &str) -> bool {
    !candidate.chars().filter(|c| c.is_alphabetic()).any(|c| {
        candidate
            .chars()
            .filter(|h| h.to_ascii_lowercase() == c.to_ascii_lowercase())
            .count()
            > 1
    })
}
