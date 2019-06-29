use na::Point2;

#[inline]
pub fn distance_squared_to(my_pos: &Point2<f64>, other_pos: &Point2<f64>) -> f64 {
    (other_pos.x - my_pos.x).powi(2) + (other_pos.y - my_pos.y).powi(2)
}

#[inline]
pub fn distance_to(my_pos: &Point2<f64>, other_pos: &Point2<f64>) -> f64 {
    distance_squared_to(my_pos, other_pos).sqrt()
}