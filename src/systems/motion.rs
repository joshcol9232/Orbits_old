use specs::prelude::*;

use crate::components::{
    Pos, Vel, Force, Mass,
};

pub struct MotionSys;

impl<'a> System<'a> for MotionSys {
    type SystemData = (
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Force>,
        ReadStorage<'a, Mass>
    );

    fn run(&mut self, (mut pos, mut vel, mut force, mass): Self::SystemData) {
        use specs::Join;

        // Update entities with force and mass
        for (vel, force, mass) in (&mut vel, &mut force, &mass).join() {
            // F = ma, a = F/m
            vel.0 += force.0/mass.0;
        }

        // Update entities with velocity
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.0 += vel.0;
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Force>();
        world.register::<Mass>();
    }
}