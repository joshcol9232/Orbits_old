use ggez::nalgebra as na;
use ggez::graphics::{self, Mesh, DrawParam, DrawMode};
use ggez::{Context, GameResult};
use ggez::timer;

use na::{
    Point2,
    Vector2,
};
use std::fmt;
use std::f64::consts::PI;
use std::time::Duration;
use crate::{
    Mobile,
    particles::planet_particles::PlanetTrailParticleSys,
    particles::ParticleSystem,
};

pub const PL_DENSITY: f64 = 5000.0;
const TRAIL_PLACEMENT_PERIOD: f64 = 0.1;
const TRAIL_NODE_LIFETIME: Duration = Duration::from_secs(2);

pub type PlanetID = u32;

#[derive(Clone)]
pub struct Planet {
    pub id: PlanetID,
    pub pos: Point2<f64>,
    vel: Vector2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub res_force: Vector2<f64>,
}

impl Planet {
    pub fn new(id: PlanetID, pos: Point2<f64>, vel: Vector2<f64>, radius: f64, m: f64) -> Planet {
        Planet {
            id,
            pos,
            vel,
            radius,
            mass: if m <= 0.0 { Self::get_mass_from_radius(radius) } else { m },
            res_force: Vector2::new(0.0, 0.0),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circ = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            self.radius as f32,
            0.05,
            graphics::WHITE
        )?;

        graphics::draw(
            ctx,
            &circ,
            DrawParam::default().dest(Point2::new(self.pos.x as f32, self.pos.y as f32))
        )?;

        Ok(())
    }

    pub fn update_physics(&mut self, dt: f64) {
        // F/m = a
        self.vel += (self.res_force / self.mass) * dt;
        self.pos += self.vel * dt;
        self.res_force = Vector2::new(0.0, 0.0);
    }

    #[inline]
    fn get_volume(r: f64) -> f64 {
        (4.0/3.0) * PI * r.powi(3)
    }

    #[inline]
    fn get_mass_from_radius(r: f64) -> f64 {
        // d = m/v => dv = m
        Self::get_volume(r) * PL_DENSITY
    }

    #[inline]
    fn get_momentum(&self) -> Vector2<f64> {
        self.vel * self.mass
    }

    pub fn collide(&mut self, other: &Self) {
        let total_momentum = self.get_momentum() + other.get_momentum();
        let total_mass = self.mass + other.mass;
        let (v_me, v_other) = (Self::get_volume(self.radius), Self::get_volume(other.radius));
        let total_vol = v_me + v_other;

        // My volume will always be bigger or the same (checked in loop)
        // Ratio of volumes
        if v_other/v_me > 0.75 { // If ratio close to 1 (both simmilar size), then pick the mid-point
            self.pos = Point2::new((self.pos.x + other.pos.x)/2.0, (self.pos.y + other.pos.y)/2.0);
        }

        self.vel = total_momentum/total_mass;
        self.radius = (((3.0/4.0) * total_vol)/PI).powf(1.0/3.0);
        self.mass = total_mass;
    }
}

impl Mobile<f64> for Planet {
    mobile_get_set_defaults!(f64);
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Planet {}

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub struct PlanetTrail {
    pub pos: Point2<f64>,   // Not a reference to planet pos, since i want it to live longer than planet
    pub dead: bool,
    particles: PlanetTrailParticleSys,
    linear_trail: Vec<TrailNode>,
    linear_node_placement_timer: f64,
}

impl PlanetTrail {
    pub fn new(pos: Point2<f64>) -> PlanetTrail {
        let mut p = PlanetTrail {
            pos,
            dead: false,
            particles: PlanetTrailParticleSys::new(),
            linear_trail: Vec::with_capacity(60),
            linear_node_placement_timer: 0.0,
        };

        p
    }

    pub fn update(&mut self, dt: f64, current_time: &Duration) {
        self.dead = self.particles.dead && self.linear_trail.len() == 0;

        self.linear_node_placement_timer += dt;
        self.particles.update(dt, current_time, &self.pos);
        self.kill_dead_nodes(current_time);

        if self.linear_node_placement_timer >= TRAIL_PLACEMENT_PERIOD {
            let num = (self.linear_node_placement_timer/TRAIL_PLACEMENT_PERIOD).round();
            self.linear_node_placement_timer -= TRAIL_PLACEMENT_PERIOD * num;
            self.place_node(current_time);
        }
    }

    pub fn draw(&self, ctx: &mut Context, current_time: &Duration) -> GameResult {
        self.particles.draw(ctx, current_time)?;
        if self.linear_trail.len() > 0 {
            println!("Drawing trail");
            self.draw_line(ctx, current_time)?;
        }
        Ok(())
    }

    fn draw_line(&self, ctx: &mut Context, current_time: &Duration) -> GameResult {
        let trail_lifetime_float = timer::duration_to_f64(TRAIL_NODE_LIFETIME);
        
        for i in 0..self.linear_trail.len()-1 {
            let alpha = 1.0 - timer::duration_to_f64(*current_time - self.linear_trail[i].time_created)/trail_lifetime_float;
            let line_mesh = Mesh::new_line(
                ctx,
                &[self.linear_trail[i].pos, self.linear_trail[i+1].pos],
                2.0,
                [0.0, 0.0, 1.0, alpha as f32].into()
            )?;

            graphics::draw(ctx, &line_mesh, DrawParam::default())?;
        }
        Ok(())
    }

    fn place_node(&mut self, current_time: &Duration) {
        self.linear_trail.push(TrailNode {
            pos: Point2::new(self.pos.x as f32, self.pos.y as f32),
            time_created: current_time.clone(),
        });
    }

    #[inline]
    fn kill_dead_nodes(&mut self, current_time: &Duration) {
        self.linear_trail.retain(|n| *current_time - n.time_created >= TRAIL_NODE_LIFETIME);
    }

    #[inline]
    pub fn particle_count(&self) -> usize {
        self.particles.particle_count()
    }


}

struct TrailNode {
    pos: Point2<f32>,
    time_created: Duration,
}

impl Into<Point2<f32>> for TrailNode {
    fn into(self) -> Point2<f32> { self.pos }
}