use std::time::Duration;

pub mod planet_particles;

pub trait ParticleSystem {
    // Updates particles and decides whever to emit again
    fn kill_particles(&mut self, current_time: &Duration);

    fn particle_count(&self) -> usize;
}

pub trait Particle: crate::Mobile<f32> {
    fn time_created(&self) -> &Duration;
    fn lifetime(&self) -> &Duration;
    fn rad(&self) -> f32;
}
