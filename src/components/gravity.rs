use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GravForceCalculated(pub bool);