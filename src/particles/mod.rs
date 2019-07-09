#[macro_use]

use na::{Point2, Vector2};
use std::time::Duration;
use rand::{Rng, rngs::ThreadRng};

use nannou::draw::Draw;
use nannou::color::{self, named};
use nannou::time::DurationF64;

const PLANET_TRAIL_VEL_LIMITS: (f32, f32) = (-5.0, 5.0);
const PLANET_TRAIL_RAD_LIMITS: (f32, f32) = (0.5, 2.0);
const PLANET_TRAIL_MAX_LIFETIME: Duration = Duration::from_secs(2);
const PLANET_TRAIL_EMMISION_PERIOD: f64 = 0.05;    // Time between emmisions

pub trait ParticleSystem {
    // Updates particles and decides whever to emit again
    fn update(amount: usize, dt: f64, current_time: &Duration);
    fn particle_count(&self) -> usize;

    fn dead(&self) -> bool;
    fn dead_mut(&mut self) -> &mut bool;
}

// Dead bool value should be a thing for all particle systems, and all should be self.dead unless specified
#[macro_export]
macro_rules! particle_system_dead_get_sets_defaults {
    () => {
        #[inline]
        fn dead(&self) -> bool { self.dead }
        #[inline]
        fn dead_mut(&mut self) -> &mut bool { &mut self.dead }
    };
}

pub trait Particle: crate::body::Mobile {
    fn time_created(&self) -> &Duration;
    fn lifetime(&self) -> &Duration;
}

#[macro_export]
macro_rules! particle_get_sets_defaults {
    () => {
        #[inline]
        fn time_created(&self) -> &Duration { &self.time_created }
        #[inline]
        fn lifetime(&self) -> &Duration { &self.lifetime }
        #[inline]
        fn pos(&self) -> &Point2<f32> { &self.pos }
        #[inline]
        fn vel(&self) -> &Vector2<f32> { &self.vel }
        #[inline]
        fn rad(&self) -> f32;
    };
}