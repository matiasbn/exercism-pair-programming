// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        0 => 0.0 as f64,
        1..=4 => f64::from(221_u16 * speed as u16),
        5..=8 => f64::from(221_u16 * speed as u16) * 0.9,
        9..=10 => f64::from(221_u16 * speed as u16) * 0.77,
        _ => 1 as f64,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
