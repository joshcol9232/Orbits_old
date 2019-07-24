use specs::prelude::*;

use euclid::{Point2D, Vector2D};

use crate::components::{Pos, shape::Shape, Vel, Mass};
use crate::tools;

pub struct CollisionSys;

impl CollisionSys {
    // Bounding box check for circles
    #[inline]
    fn aabb_circles(p1: &Point2D<f64, f64>, p2: &Point2D<f64, f64>, r1: f64, r2: f64) -> bool {
        let total_rad = r1 + r2;
        p2.x - p1.x <= total_rad && p2.y - p1.y <= total_rad
    }

    #[inline]
    fn circle_collision(p1: &Point2D<f64, f64>, p2: &Point2D<f64, f64>, r1: f64, r2: f64) -> bool {
        Self::aabb_circles(p1, p2, r1, r2) && (*p2 - *p1).square_length() <= (r1 + r2).powi(2)
    }
}

impl<'a> System<'a> for CollisionSys {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Shape>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Mass>,
    );

    fn run(&mut self, (entities, mut shape, positions, mut velocities, mut masses): Self::SystemData) {
        // use std::cell::RefCell;

        // // Only store things immediately needed.
        // let objects: Vec<RefCell<(&Pos, &mut Shape, Entity)>> = (&positions, &mut shape, &*entities).join()
        //     .map(|item| RefCell::new(item)).collect();

        // for i in 0..objects.len()-1 {
        //     for j in i+1..objects.len() {
        //         let mut me = objects[i].borrow_mut();
        //         let mut other = objects[j].borrow_mut();

        //         // Circle object on circle object
        //         if let Shape::Circle(r1) = &me.1 {
        //             if let Shape::Circle(r2) = &other.1 {
        //                 // If both circles
        //                 if Self::circle_collision(&(me.0).0, &(other.0).0, *r1, *r2) {
        //                     println!("Colliding! : {}, {}", (me.0).0, (other.0).0);
        //                 }
        //             }
        //         }
        //     }
        // }
    }
}