use ggez::nalgebra as na;
use na::{Point2, Vector2};
use rand::{Rng, rngs::ThreadRng};
use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, DrawMode, DrawParam};
use ggez::timer;
// use nannou::draw::Draw;
// use nannou::prelude::DurationF64;
use std::time::Duration;
use std::collections::VecDeque;

use super::{ParticleSystem, Particle};
use crate::Mobile;

const PARTICLE_VEL_LIMITS: (f32, f32) = (-5.0, 5.0);
const PARTICLE_RAD_LIMITS: (f32, f32) = (0.5, 3.0);
const PARTICLE_LIFETIME: Duration = Duration::from_millis(2000);
const PARTICLE_EMMISION_PERIOD: f64 = 0.08; // Time between emmisions

pub struct PlanetTrailParticleSys {
    particles: VecDeque<PlanetTrailParticle>,
    rand_thread: ThreadRng,
    emmision_timer: f64,
}

impl PlanetTrailParticleSys {
    pub fn new() -> PlanetTrailParticleSys {
        let mut p = PlanetTrailParticleSys {
            particles: VecDeque::with_capacity(26),
            rand_thread: rand::thread_rng(),
            emmision_timer: 0.0,
        };

        p.add_particle(&Duration::new(0, 0), &Point2::new(0.0, 0.0));

        p
    }

    fn add_particle(&mut self, current_time: &Duration, pos: &Point2<f32>) {
        self.particles.push_back(
            PlanetTrailParticle::new(
                *pos,
                Vector2::new(
                    self.rand_thread.gen_range(PARTICLE_VEL_LIMITS.0, PARTICLE_VEL_LIMITS.1),
                    self.rand_thread.gen_range(PARTICLE_VEL_LIMITS.0, PARTICLE_VEL_LIMITS.1)
                ),
                self.rand_thread.gen_range(PARTICLE_RAD_LIMITS.0, PARTICLE_RAD_LIMITS.1),
                current_time.clone()
            )
        );
    }

    #[inline]
    fn emit(&mut self, amount: usize, current_time: &Duration, pos: &Point2<f32>) {
        for _ in 0..amount {
            self.add_particle(current_time, pos);
        }
    }

    pub fn draw(&self, ctx: &mut Context, current_time: &Duration) -> GameResult {
        for p in self.particles.iter() {
            if p.time_created > Duration::new(0, 0) {
                let alpha: f64 = 1.0 - (timer::duration_to_f64(*current_time - p.time_created)/timer::duration_to_f64(PARTICLE_LIFETIME));

                let circ = Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    Point2::new(0.0, 0.0),
                    p.rad,
                    0.05,
                    /* Particle colour:
                        -- Pinkish d824e5
                        -- Mint/Green 23ddaf
                    */
                    [0.13671875, 0.86328125, 0.68359375, alpha as f32].into()
                )?;

                graphics::draw(ctx, &circ, DrawParam::default().dest(Point2::new(p.pos.x as f32, p.pos.y as f32)))?;
            }
        }

        Ok(())
    }


    pub fn update_emmision(&mut self, dt: f64, current_time: &Duration, pos: &Point2<f32>) {
        self.emmision_timer += dt;

        if self.emmision_timer >= PARTICLE_EMMISION_PERIOD {
            let num = (self.emmision_timer/PARTICLE_EMMISION_PERIOD).round();
            self.emmision_timer -= PARTICLE_EMMISION_PERIOD * num;

            self.emit(num as usize, current_time, pos);
        }
    }

    pub fn update_particles(&mut self, dt: f32, current_time: &Duration) {
        self.kill_particles(current_time);
        
        for p in self.particles.iter_mut() {
            p.update_pos(dt);
        }
    }
}

impl ParticleSystem for PlanetTrailParticleSys {
    particle_system_defaults!(PARTICLE_LIFETIME);
}

struct PlanetTrailParticle {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    rad: f32,
    time_created: Duration,
}

impl PlanetTrailParticle {
    fn new(pos: Point2<f32>, vel: Vector2<f32>, rad: f32, time: Duration) -> PlanetTrailParticle {
        PlanetTrailParticle {
            pos,
            vel,
            rad,
            time_created: time,
        }
    }
}

impl Mobile<f32> for PlanetTrailParticle {
    mobile_get_set_defaults!(f32);
}

impl Particle for PlanetTrailParticle {
    particle_set_get_defaults!(&PARTICLE_LIFETIME);
}