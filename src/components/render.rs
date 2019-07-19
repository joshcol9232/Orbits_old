use specs::prelude::*;

// For telling what thing to render.

#[derive(Default)]
pub struct PlanetRender;

impl Component for PlanetRender {
    type Storage = NullStorage<Self>;
}