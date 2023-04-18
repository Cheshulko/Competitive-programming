pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (((square(64) as u128) << 1) - 1) as u64
}
