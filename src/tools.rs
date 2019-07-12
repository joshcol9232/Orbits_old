use ggez::nalgebra as na;
use na::{Point2, RealField, Vector2};

use crate::GRAV_CONSTANT;

#[inline]
pub fn distance_squared_to<T: RealField>(my_pos: &Point2<T>, other_pos: &Point2<T>) -> T {
    (other_pos.x - my_pos.x).powi(2) + (other_pos.y - my_pos.y).powi(2)
}

#[inline]
pub fn distance_to<T: RealField>(my_pos: &Point2<T>, other_pos: &Point2<T>) -> T {
    distance_squared_to(my_pos, other_pos).sqrt()
}

#[inline]
pub fn newtonian_grav(m1: f64, m2: f64, pos1: &Point2<f64>, pos2: &Point2<f64>) -> Vector2<f64> {
    let dist_vec = Vector2::new(pos2.x - pos1.x, pos2.y - pos1.y);
    let force = (GRAV_CONSTANT * m1 * m2) / (dist_vec.x.powi(2) + dist_vec.y.powi(2));
    let angle = dist_vec.y.atan2(dist_vec.x);

    Vector2::new(force * angle.cos(), force * angle.sin())
}
