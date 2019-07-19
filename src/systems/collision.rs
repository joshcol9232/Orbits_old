use specs::prelude::*;

use crate::components::{Pos, shape::Shape};

pub struct CollisionSys;

impl CollisionSys {
    // Bounding box check for circles
    fn aabb_circles(p1: &Point2D<f64, f64>, p2: &Point2D<f64, f64>, r1: f64, r2: f64) -> bool {
        
    }
}

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Shape>,
    );

    fn run(&mut self, (pos, shape): Self::SystemData) {
        match shape {
            Shape::Circle(radius) => {

            },
            _ => ()
        }
    }
}