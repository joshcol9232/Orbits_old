use std::f64::consts::PI;

#[inline]
pub fn sphere_volume(radius: f64) -> f64 {
    (4.0/3.0) * std::f64::consts::PI * radius.powi(3)
}