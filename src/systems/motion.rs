use specs::prelude::*;

use euclid::Vector2D;

use crate::{
    time::DeltaTime,
    components::{
        Pos, Vel, Force, Mass,
    },
};

pub struct MotionSys;

impl<'a> System<'a> for MotionSys {
    type SystemData = (
        Read<'a, DeltaTime>,    // Get delta time
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Force>,
        ReadStorage<'a, Mass>
    );

    fn run(&mut self, (dt, mut pos, mut vel, mut force, mass): Self::SystemData) {
        // Update entities with force and mass
        for (vel, force, mass) in (&mut vel, &mut force, &mass).join() {
            // F = ma, a = F/m
            vel.0 += (force.0/mass.0) * dt.0;
            // Reset resultant force
            force.0 = Vector2D::zero();
        }

        // Update entities with velocity
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0 * dt.0;
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Force>();
        world.register::<Mass>();
    }
}