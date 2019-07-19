use specs::prelude::*;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum Shape {
    Circle(f64),
}