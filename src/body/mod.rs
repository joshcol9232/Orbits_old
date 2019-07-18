pub mod planet;

use ggez::graphics::{self, DrawMode, DrawParam, Mesh};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

use crate::tools;
use na::{Point2, Vector2, RealField};
use serde::{Serialize, Deserialize};

use std::f64::consts::PI;
use std::fmt;


pub const PL_DENSITY: f64 = 5000.0;

// For mobile objects
pub trait Mobile<T: RealField> {
    fn pos(&self) -> &Point2<T>;
    fn pos_mut(&mut self) -> &mut Point2<T>;
    fn vel(&self) -> &Vector2<T>;
    fn vel_mut(&mut self) -> &mut Vector2<T>;

    fn update_pos(&mut self, dt: T) {
        let vel = *self.vel();
        *self.pos_mut() += vel * dt;
    }
}

pub type BodyID = u32;

#[derive(Clone)]
pub struct Body {
    pub id: BodyID,
    pub pos: Point2<f64>,
    vel: Vector2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub res_force: Vector2<f64>,
}

impl Body {
    pub fn new(id: BodyID, pos: Point2<f64>, vel: Vector2<f64>, radius: f64, m: f64) -> Body {
        Body {
            id,
            pos,
            vel,
            radius,
            mass: if m <= 0.0 {
                Self::get_mass_from_radius(radius)
            } else {
                m
            },
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
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &circ,
            DrawParam::default().dest(cast_point2_to_f32!(self.pos)),
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
        (4.0 / 3.0) * PI * r.powi(3)
    }

    // Returns radius
    #[inline]
    fn inverse_volume(vol: f64) -> f64 {
        (((3.0 / 4.0) * vol) / PI).powf(1.0 / 3.0)
    }

    #[inline]
    fn get_mass_from_radius(r: f64) -> f64 {
        // d = m/v => dv = m
        Self::get_volume(r) * PL_DENSITY
    }

    #[inline]
    fn get_radius_from_mass(mass: f64) -> f64 {
        Self::inverse_volume(mass/PL_DENSITY)
    }

    #[inline]
    fn get_momentum(&self) -> Vector2<f64> {
        self.vel * self.mass
    }

    pub fn collide(&mut self, other: &Self) {
        let total_momentum = self.get_momentum() + other.get_momentum();
        let total_mass = self.mass + other.mass;
        let (v_me, v_other) = (
            Self::get_volume(self.radius),
            Self::get_volume(other.radius),
        );
        let total_vol = v_me + v_other;

        // My volume will always be bigger or the same (checked in loop)
        // Ratio of volumes
        if v_other / v_me > 0.75 {
            // If ratio close to 1 (both simmilar size), then pick the mid-point
            self.pos = Point2::new(
                (self.pos.x + other.pos.x) / 2.0,
                (self.pos.y + other.pos.y) / 2.0,
            );
        }

        self.vel = total_momentum / total_mass;
        self.radius = Self::inverse_volume(total_vol);
        self.mass = total_mass;
    }

    // ratio is percentage of planet to keep.
    pub fn split(&mut self, ratio: f64, new_id: BodyID, split_momentum: Vector2<f64>, split_angle: f64) -> Body {
        let my_new_mass = self.mass * ratio;
        let new_pl_mass = self.mass - my_new_mass;
        self.mass = my_new_mass;

        self.radius = Self::get_radius_from_mass(self.mass);
        let new_pl_radius = Self::get_radius_from_mass(new_pl_mass);
        let new_pl_vel = (split_momentum/new_pl_mass) + self.vel;       // + self.vel due to relativity

        // Gonna keep self.pos the same
        let new_pl_pos: Point2<f64> = {
            // let dr = old_radius - new_pl_radius;
            // tools::get_components(dr, split_angle);
            let mag = self.radius + new_pl_radius + 2.0;    // Don't touch otherwise will cause chain reaction
            (self.pos + tools::get_components(mag, split_angle)).into()
        };

        self.vel -= split_momentum/self.mass;

        //println!("New pl vel: {} {}", self.vel, new_pl_vel);

        Body::new(new_id, new_pl_pos, new_pl_vel, new_pl_radius, new_pl_mass)
    }
}

impl Mobile<f64> for Body {
    mobile_get_set_defaults!(f64);
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Body {}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl From<&BodySaveData> for Body {
    fn from(pl_save: &BodySaveData) -> Self {
        Body {
            id: pl_save.id,
            pos: Point2::new(pl_save.pos_x, pl_save.pos_y),
            vel: Vector2::new(pl_save.vel_x, pl_save.vel_y),
            radius: pl_save.radius,
            mass: pl_save.mass,
            res_force: Vector2::new(0.0, 0.0),
        }
    }
}


pub enum BodyType {
    Planet,
    Star,
}


#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct BodySaveData {
    pub id: BodyID,
    pub pos_x: f64,
    pub pos_y: f64,
    pub vel_x: f64,
    pub vel_y: f64,
    pub radius: f64,
    pub mass: f64
}

impl From<std::cell::Ref<'_, Body>> for BodySaveData {
    fn from(pl: std::cell::Ref<'_, Body>) -> Self {
        BodySaveData {
            id: pl.id,
            pos_x: pl.pos.x,
            pos_y: pl.pos.y,
            vel_x: pl.vel.x,
            vel_y: pl.vel.y,
            radius: pl.radius,
            mass: pl.mass,
        }
    }
}