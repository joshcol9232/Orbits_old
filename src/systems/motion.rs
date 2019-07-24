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
        (&mut vel, &mut force, &mass).par_join()
            .for_each(|(vel, force, mass)| {
                // F = ma, a = F/m
                vel.0 += (force.0/mass.0) * dt.0;
                // Reset resultant force
                force.0 = Vector2D::zero();
            });

        (&mut pos, &vel).par_join()
            .for_each(|(pos, vel)| {
                pos.0 += vel.0 * dt.0;
            });
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Force>();
        world.register::<Mass>();
    }
}