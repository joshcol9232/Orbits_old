pub mod shape;
pub mod render;

use specs::prelude::*;
use euclid::{Point2D, Vector2D};

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct Pos(pub Point2D<f64, f64>);

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct Vel(pub Vector2D<f64, f64>);

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(DenseVecStorage)]
pub struct Force(pub Vector2D<f64, f64>);

#[derive(Component, Debug, Default, Copy, Clone)]
#[storage(VecStorage)]
pub struct Mass(pub f64);