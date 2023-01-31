pub fn square(s: u32) -> u64 {
    assert!(s <= 64 && s > 0, "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64).fold(0u64, |total, num| total + square(num))
}
