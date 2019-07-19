use specs::prelude::*;

use crate::{
    components::{
        Pos, Force, Mass,
        gravity::GravForceCalculated,
    },
};

pub struct GravitySys;

impl<'a> System<'a> for GravitySys {
    type SystemData = (
        WriteStorage<'a, Force>,
        WriteStorage<'a, GravForceCalculated>,
        ReadStorage<'a, Mass>,
        ReadStorage<'a, Pos>,
    );

    fn run(&mut self, (mut res_force, mut calculated, mass, pos): Self::SystemData) {
        use specs::Join;

        for (res_force, calculated, mass, pos) in (&mut res_force, &mut calculated, &mass, &pos).join() {
            if !calculated.0 {
                println!("EGG: {:?}, {:?}, {:?}", res_force, mass, pos);
                calculated.0 = true;
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        world.register::<GravForceCalculated>();
    }
}