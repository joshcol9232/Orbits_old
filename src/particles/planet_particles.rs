use ggez::nalgebra as na;
use na::{Point2, Vector2};
use rand::{Rng, rngs::ThreadRng};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, DrawMode, DrawParam};
use ggez::timer;
// use nannou::draw::Draw;
// use nannou::prelude::DurationF64;
use std::time::Duration;

use super::{ParticleSystem, Particle};
use crate::Mobile;

const PLANET_TRAIL_VEL_LIMITS: (f32, f32) = (-5.0, 5.0);
const PLANET_TRAIL_RAD_LIMITS: (f32, f32) = (0.5, 3.0);
const PLANET_TRAIL_LIFETIME: Duration = Duration::from_millis(2500); // 2.5 secs
const PLANET_TRAIL_EMMISION_PERIOD: f64 = 0.08; // Time between emmisions

pub struct PlanetTrailParticleSys {
    particles: Vec<PlanetTrailParticle>,
    rand_thread: ThreadRng,
    emmision_timer: f64,
    pub dead: bool,
}

impl PlanetTrailParticleSys {
    pub fn new() -> PlanetTrailParticleSys {
        let mut p = PlanetTrailParticleSys {
            particles: Vec::with_capacity(61),
            rand_thread: rand::thread_rng(),
            emmision_timer: 0.0,
            dead: false,
        };

        p.add_particle(&Duration::new(0, 0), &Point2::new(0.0, 0.0));

        p
    }

    fn add_particle(&mut self, current_time: &Duration, pos: &Point2<f64>) {
        self.particles.push(
            PlanetTrailParticle::new(
                Point2::new(pos.x as f32, pos.y as f32),
                Vector2::new(
                    self.rand_thread.gen_range(PLANET_TRAIL_VEL_LIMITS.0, PLANET_TRAIL_VEL_LIMITS.1),
                    self.rand_thread.gen_range(PLANET_TRAIL_VEL_LIMITS.0, PLANET_TRAIL_VEL_LIMITS.1)
                ),
                self.rand_thread.gen_range(PLANET_TRAIL_RAD_LIMITS.0, PLANET_TRAIL_RAD_LIMITS.1),
                current_time.clone(),
                PLANET_TRAIL_LIFETIME
            )
        );
    }

    #[inline]
    fn emit(&mut self, amount: usize, current_time: &Duration, pos: &Point2<f64>) {
        for _ in 0..amount {
            self.add_particle(current_time, pos);
        }
    }

    pub fn draw(&self, ctx: &mut Context, current_time: &Duration) -> GameResult {
        for p in self.particles.iter() {
            if p.time_created < *current_time {
                let alpha: f64 = 1.0 - (timer::duration_to_f64(*current_time - p.time_created)/timer::duration_to_f64(p.lifetime));

                let circ = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Point2::new(0.0, 0.0),
                    p.rad,
                    0.05,
                    [0.5, 0.5, 1.0, alpha as f32].into()
                )?;

                graphics::draw(ctx, &circ, DrawParam::default().dest(Point2::new(p.pos.x as f32, p.pos.y as f32)))?;
            }
        }

        Ok(())
    }


    pub fn update(&mut self, dt: f64, current_time: &Duration, pos: &Point2<f64>) {
        self.kill_particles(current_time);
        for p in self.particles.iter_mut() {
            p.update_pos(dt as f32);
        }

        if !self.dead {
            self.emmision_timer += dt;

            if self.emmision_timer >= PLANET_TRAIL_EMMISION_PERIOD {
                let num = (self.emmision_timer/PLANET_TRAIL_EMMISION_PERIOD).round();
                self.emmision_timer -= PLANET_TRAIL_EMMISION_PERIOD * num;

                self.emit(num as usize, current_time, pos);
            }
        }
    }
}

impl ParticleSystem for PlanetTrailParticleSys {
    particle_system_defaults!();
}

struct PlanetTrailParticle {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    rad: f32,
    time_created: Duration,
    lifetime: Duration,
}

impl PlanetTrailParticle {
    fn new(pos: Point2<f32>, vel: Vector2<f32>, rad: f32, time: Duration, lifetime: Duration) -> PlanetTrailParticle {
        PlanetTrailParticle {
            pos,
            vel,
            rad,
            time_created: time,
            lifetime,
        }
    }
}

impl Mobile<f32> for PlanetTrailParticle {
    mobile_get_set_defaults!(f32);
}

impl Particle for PlanetTrailParticle {
    particle_set_get_defaults!();
}