// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed{
        1..=4 => (f64::from(speed) * f64::from(221)),
        5..=8 => ((f64::from(speed) * f64::from(221)) * 0.90 ),
        9..=10 => ((f64::from(speed) * f64::from(221)) * 0.77 ),
        _ => 0.0
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    

  let prod_rate_per_hr = production_rate_per_hour(speed).trunc() as u32;
    
   prod_rate_per_hr / 60_u32
}
