
#[macro_use]

use na::{Point2, Vector2};
use std::time::Duration;

#[macro_export]
macro_rules! particle_system_defaults {
    () => {
        #[inline]
        fn kill_particles(&mut self, current_time: &Duration) { self.particles.retain(|p| *current_time - p.time_created < p.lifetime) }
        #[inline]
        fn particle_count(&self) -> usize { self.particles.len() }
        #[inline]
        fn dead(&self) -> bool { self.dead }
        #[inline]
        fn dead_mut(&mut self) -> &mut bool { &mut self.dead }
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

pub mod planet_trail;

use nannou::draw::Draw;
use nannou::color::{self, named};
use nannou::time::DurationF64;

const PLANET_TRAIL_VEL_LIMITS: (f32, f32) = (-5.0, 5.0);
const PLANET_TRAIL_RAD_LIMITS: (f32, f32) = (0.5, 2.0);
const PLANET_TRAIL_MAX_LIFETIME: Duration = Duration::from_secs(2);
const PLANET_TRAIL_EMMISION_PERIOD: f64 = 0.05;    // Time between emmisions

pub trait ParticleSystem {
    // Updates particles and decides whever to emit again
    fn update(&mut self, dt: f64, current_time: &Duration);
    fn kill_particles(&mut self, current_time: &Duration);

    fn particle_count(&self) -> usize;
    fn dead(&self) -> bool;
    fn dead_mut(&mut self) -> &mut bool;
}

pub trait Particle: crate::Mobile<f32> {
    fn time_created(&self) -> &Duration;
    fn lifetime(&self) -> &Duration;
    fn rad(&self) -> f32;
}