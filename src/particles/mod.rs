use std::time::Duration;

//use crate::macros::kill_objects_with_lifetime;

#[macro_export]
macro_rules! particle_system_defaults {
    ($max_lifetime:expr) => {
        #[inline]
        fn kill_particles(&mut self, current_time: &Duration) {
            kill_objects_with_lifetime!(self.particles, current_time, $max_lifetime);
        }
        #[inline]
        fn particle_count(&self) -> usize { self.particles.len() }
    };
}

#[macro_export]
macro_rules! particle_set_get_defaults {
    () => {
        #[inline]
        fn time_created(&self) -> &std::time::Duration { &self.time_created }
        #[inline]
        fn lifetime(&self) -> &std::time::Duration { &self.lifetime }
        #[inline]
        fn rad(&self) -> f32 { self.rad }
    };
}

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