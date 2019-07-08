use na::{
    Point2,
    Vector2
};
use nannou::{
    draw::Draw,
    color::named
};
use std::fmt;
use std::f64::consts::PI;

pub const PL_DENSITY: f64 = 5000.0;

pub type BodyID = u32;

#[derive(Clone)]
pub struct Body {
    pub id: BodyID,
    pub body_type: BodyType,
    pub pos: Point2<f64>,
    vel: Vector2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub res_force: Vector2<f64>,
}

impl Body {
    pub fn new(id: BodyID, body_type: BodyType, pos: Point2<f64>, vel: Vector2<f64>, radius: f64, m: f64) -> Body {
        Body {
            id,
            body_type,
            pos,
            vel,
            radius,
            mass: if m <= 0.0 { Self::get_mass_from_radius(radius) } else { m },
            res_force: Vector2::new(0.0, 0.0),
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .radius(self.radius as f32)
            .x_y(self.pos.x as f32, self.pos.y as f32)
            .color(named::WHITE);
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

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Body {}

impl fmt::Debug for Body {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Body: {}", self.id)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BodyType {
    Planet,
    Star,
}