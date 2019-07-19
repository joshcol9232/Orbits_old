use specs::prelude::*;

use euclid::{Point2D, Vector2D};

use crate::{
    components::{
        Pos, Force, Mass,
    },
};

pub const G: f64 = 0.001;

pub struct GravitySys;

impl GravitySys {
    fn newtonian(p1: &Point2D<f64, f64>, p2: &Point2D<f64, f64>, mass1: f64, mass2: f64) -> Vector2D<f64, f64> {
        let dist_vec: Vector2D<f64, f64> = *p2 - *p1;
        let angle = dist_vec.y.atan2(dist_vec.x);
        let dist_squared = dist_vec.square_length();

        let mag = (G * mass1 * mass2)/dist_squared;

        Vector2D::new(mag * angle.cos(), mag * angle.sin())
    }
}

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        WriteStorage<'a, Force>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Pos>,
    );

    fn run(&mut self, (mut res_force, mass, pos): Self::SystemData) {
        use std::cell::RefCell;

        let objects: Vec<RefCell<(&mut Force, &Mass, &Pos)>> = (&mut res_force, &mass, &pos).join()
            .map(|item| RefCell::new(item))
            .collect();

        for i in 0..objects.len()-1 {
            for j in i+1..objects.len() {
                let mut me = objects[i].borrow_mut();
                let mut other = objects[j].borrow_mut();
                // 0 is force, 1 is mass, 2 is pos
                let (my_pos, other_pos) = (&(other.2).0, &(me.2).0);
                let (my_mass, other_mass) = ((me.1).0, (other.1).0);

                let grav_force = Self::newtonian(my_pos, other_pos, my_mass, other_mass);

                // Equal and opposite force
                (me.0).0 -= grav_force;
                (other.0).0 += grav_force;
            }
        }
    }

    // fn setup(&mut self, world: &mut World) {
    // }
}