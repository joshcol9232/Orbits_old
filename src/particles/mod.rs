pub mod planet_particles;

use std::time::Duration;
use crate::body::Mobile;

pub trait ParticleSystem {
    // Updates particles and decides whever to emit again
    fn kill_particles(&mut self, current_time: &Duration);

    fn particle_count(&self) -> usize;
}

pub trait Particle: Mobile<f32> {
    fn time_created(&self) -> &Duration;
    fn lifetime(&self) -> &Duration;
    fn rad(&self) -> f32;
}
