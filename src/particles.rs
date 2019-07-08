use na::{Point2, Vector2};
use std::time::Duration;
use rand::{Rng, rngs::ThreadRng};
use nannou::draw::Draw;
use nannou::color::named;

const PLANET_TRAIL_VEL_LIMITS: (f32, f32) = (-5.0, 5.0);
const PLANET_TRAIL_RAD_LIMITS: (f32, f32) = (1.0, 5.0);
const PLANET_TRAIL_MAX_LIFETIME: Duration = Duration::from_secs(2);
const PLANET_TRAIL_EMMISION_PERIOD: f64 = 0.05;    // Time between emmisions

pub struct ParticleSystem {
    particles: Vec<Particle>,
    pub pos: Point2<f64>,
    rand_thread: ThreadRng,
    sys_type: ParticleSysType,
    emmision_timer: f64,
    pub dead: bool,
}

impl ParticleSystem {
    pub fn new(pos: Point2<f64>, system_type: ParticleSysType) -> ParticleSystem {
        let mut p = ParticleSystem {
            particles: Vec::with_capacity(
                match system_type {
                    ParticleSysType::PlanetTrail => 41,
                }
            ),
            pos,
            rand_thread: rand::thread_rng(),
            sys_type: system_type,
            emmision_timer: 0.0,
            dead: false,
        };

        p.add_particle(&Duration::new(0, 0));

        p
    }

    pub fn get_particle_count(&self) -> usize {
        self.particles.len()
    }

    fn add_particle(&mut self, current_time: &Duration) {
        let temp_pos = Point2::new(self.pos.x as f32, self.pos.y as f32);
        let (pos, vel, rad, lifetime) = match self.sys_type {
            ParticleSysType::PlanetTrail => {
                (
                    temp_pos,
                    Vector2::new(
                        self.rand_thread.gen_range(PLANET_TRAIL_VEL_LIMITS.0, PLANET_TRAIL_VEL_LIMITS.1),
                        self.rand_thread.gen_range(PLANET_TRAIL_VEL_LIMITS.0, PLANET_TRAIL_VEL_LIMITS.1)
                    ),
                    self.rand_thread.gen_range(PLANET_TRAIL_RAD_LIMITS.0, PLANET_TRAIL_RAD_LIMITS.1),
                    PLANET_TRAIL_MAX_LIFETIME
                )
            },
        };

        self.particles.push(
            Particle::new(pos, vel, rad, current_time.clone(), lifetime)
        );
    }

    #[inline]
    pub fn emit(&mut self, amount: usize, current_time: &Duration) {
        for _ in 0..amount {
            self.add_particle(current_time);
        }
    }

    pub fn display(&self, draw: &Draw) {
        match self.sys_type {
            ParticleSysType::PlanetTrail => {
                for p in self.particles.iter() {
                    draw.ellipse()
                        .radius(p.rad)
                        .x_y(p.pos.x, p.pos.y)
                        .color(named::BLUE);
                }
            },
        }
    }

    pub fn update(&mut self, dt: f64, current_time: &Duration) {
        println!("len: {}", self.particles.len());
        self.kill_particles(current_time);
        for p in self.particles.iter_mut() {
            p.update(dt as f32);
        }

        if !self.dead {
            self.emmision_timer += dt;
            
            let time_lim = match self.sys_type {
                ParticleSysType::PlanetTrail => PLANET_TRAIL_EMMISION_PERIOD,
            };

            if self.emmision_timer >= time_lim {
                let num = (self.emmision_timer/time_lim).round();
                self.emmision_timer -= time_lim * num;

                self.emit(num as usize, current_time);
            }
        }
    }

    #[inline]
    pub fn kill_particles(&mut self, current_time: &Duration) {
        self.particles.retain(|p| *current_time - p.time_created < p.lifetime);
    }
}

struct Particle {
    pos: Point2<f32>,
    vel: Vector2<f32>,
    rad: f32,
    time_created: Duration,
    lifetime: Duration,
}

impl Particle {
    fn new(pos: Point2<f32>, vel: Vector2<f32>, rad: f32, time: Duration, lifetime: Duration) -> Particle {
        Particle {
            pos,
            vel,
            rad,
            time_created: time,
            lifetime,
        }
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.vel * dt as f32;
    }
}

pub enum ParticleSysType {
    PlanetTrail,
}