use ggez::graphics::{self, DrawMode, DrawParam, Mesh, spritebatch};
use ggez::nalgebra as na;
use ggez::timer;
use ggez::{Context, GameResult};
use na::{Point2, Vector2};
use rand::{rngs::ThreadRng, Rng};

use std::collections::VecDeque;
use std::time::Duration;

use super::{Particle, ParticleSystem};
use crate::Mobile;

const PARTICLE_VEL_LIMIT: f32 = 5.0;
const PARTICLE_RAD_LIMITS: (f32, f32) = (5.0, 10.0);
const PARTICLE_LIFETIME: Duration = Duration::from_millis(1500);
const PARTICLE_EMMISION_PERIOD: f64 = 0.02; // Time between emmisions

const SMOKE_IMAGE_DIMENSIONS: [usize; 2] = [512, 512];

pub struct PlanetTrailParticleSys {
    particles: VecDeque<PlanetTrailParticle>,
    rand_thread: ThreadRng,
    emmision_timer: f64,
}

impl PlanetTrailParticleSys {
    pub fn new() -> PlanetTrailParticleSys {
        // Expected max particles = particle_lifetime/particle_emmision_period + 1
        const EXPECTED_MAX_PARTICLE_NUM: usize = 76;

        let mut p = PlanetTrailParticleSys {
            particles: VecDeque::with_capacity(EXPECTED_MAX_PARTICLE_NUM),
            rand_thread: rand::thread_rng(),
            emmision_timer: 0.0,
        };

        p.add_particle(&Duration::new(0, 0), &Point2::new(0.0, 0.0));

        p
    }

    fn add_particle(&mut self, current_time: &Duration, pos: &Point2<f32>) {
        const TWO_PI: f32 = std::f32::consts::PI * 2.0;

        self.particles.push_back(PlanetTrailParticle::new(
            *pos,
            Vector2::new(
                self.rand_thread
                    .gen_range(-PARTICLE_VEL_LIMIT, PARTICLE_VEL_LIMIT),
                self.rand_thread
                    .gen_range(-PARTICLE_VEL_LIMIT, PARTICLE_VEL_LIMIT),
            ),
            self.rand_thread
                .gen_range(PARTICLE_RAD_LIMITS.0, PARTICLE_RAD_LIMITS.1),
            self.rand_thread.gen::<f32>() * TWO_PI,
            current_time.clone(),
        ));
    }

    #[inline]
    fn emit(&mut self, amount: usize, current_time: &Duration, pos: &Point2<f32>) {
        for _ in 0..amount {
            self.add_particle(current_time, pos);
        }
    }

    pub fn draw(&self, ctx: &mut Context, current_time: &Duration, batch: &mut spritebatch::SpriteBatch) -> GameResult {
        const SCALE: [f32; 2] = [1.0/SMOKE_IMAGE_DIMENSIONS[0] as f32, 1.0/SMOKE_IMAGE_DIMENSIONS[1] as f32];

        for p in self.particles.iter() {
            if p.time_created > Duration::new(0, 0) {
                let alpha: f64 = 1.0
                    - (timer::duration_to_f64(*current_time - p.time_created)
                        / timer::duration_to_f64(PARTICLE_LIFETIME));

                // let circ = Mesh::new_circle(
                //     ctx,
                //     DrawMode::fill(),
                //     Point2::new(0.0, 0.0),
                //     p.rad,
                //     0.05,
                //     /* Particle colour:
                //         -- Pinkish d824e5
                //         -- Mint/Green 23ddaf
                //     */
                //     [0.13671875, 0.86328125, 0.68359375, alpha as f32].into(),
                // )?;
                // graphics::draw(ctx, &circ, DrawParam::new().dest(cast_point2_to_f32!(p.pos)))?;

                let params = DrawParam::new()
                        .dest(cast_point2_to_f32!(p.pos))
                        .offset([0.5, 0.5])
                        .scale([SCALE[0] * p.rad, SCALE[1] * p.rad])
                        .rotation(p.rotation)
                        .color([0.15671875, 0.88328125, 0.72359375, alpha as f32].into());
                
                batch.add(params);
            }
        }

        Ok(())
    }

    pub fn update_emmision(&mut self, dt: f64, current_time: &Duration, pos: &Point2<f32>) {
        self.emmision_timer += dt;

        if self.emmision_timer >= PARTICLE_EMMISION_PERIOD {
            let num = (self.emmision_timer / PARTICLE_EMMISION_PERIOD).round();
            self.emmision_timer -= PARTICLE_EMMISION_PERIOD * num;

            self.emit(num as usize, current_time, pos);
        }
    }

    #[inline]
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
    rotation: f32,
    time_created: Duration,
}

impl PlanetTrailParticle {
    fn new(pos: Point2<f32>, vel: Vector2<f32>, rad: f32, rotation: f32, time: Duration) -> PlanetTrailParticle {
        PlanetTrailParticle {
            pos,
            vel,
            rad,
            rotation,
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
