use std::time::Duration;

#[macro_export]
macro_rules! particle_system_defaults {
    () => {
        #[inline]
        fn kill_particles(&mut self, current_time: &Duration) { self.particles.retain(|p| *current_time - p.time_created < p.lifetime) }
        #[inline]
        fn particle_count(&self) -> usize { self.particles.len() }
        #[inline]
        fn parent_dead(&self) -> bool { self.parent_dead }
        #[inline]
        fn parent_dead_mut(&mut self) -> &mut bool { &mut self.parent_dead }
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
    fn parent_dead(&self) -> bool;
    fn parent_dead_mut(&mut self) -> &mut bool;
}

pub trait Particle: crate::Mobile<f32> {
    fn time_created(&self) -> &Duration;
    fn lifetime(&self) -> &Duration;
    fn rad(&self) -> f32;
}