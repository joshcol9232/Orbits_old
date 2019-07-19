pub mod shape;
pub mod render;

use specs::prelude::*;
use euclid::{Point2D, Vector2D};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Pos(pub Point2D<f64, f64>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Vel(pub Vector2D<f64, f64>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Force(pub Vector2D<f64, f64>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub f64);