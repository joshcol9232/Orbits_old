use na::{
    Point2,
    Vector2
};
use nannou::{
    draw::Draw,
    color::named
};
use std::{
    fmt,
    f64::consts::PI,
    ops::{Add, AddAssign},
};

pub const PL_DENSITY: f64 = 5000.0;
pub const GRAV_CONSTANT: f64 = 0.001;

#[derive(Clone)]
pub struct Planet {
    pub id: u32,
    pub pos: Point2<f64>,
    vel: Vector2<f64>,
    pub radius: f64,
    pub mass: f64,
    pub res_force: Vector2<f64>,
    trail: Vec<na::Point2>,
}

impl Planet {
    pub fn new(id: u32, pos: Point2<f64>, vel: Vector2<f64>, radius: f64, m: f64) -> Planet {
        Planet {
            id,
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

    pub fn update(&mut self, dt: f64) {
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
}

impl AddAssign<&Planet> for Planet {
    fn add_assign(&mut self, other: &Self) {
        let total_momentum = self.get_momentum() + other.get_momentum();
        let total_mass = self.mass + other.mass;
        let total_vol = Self::get_volume(self.radius) + Self::get_volume(other.radius);

        self.pos = Point2::new((self.pos.x + other.pos.x)/2.0, (self.pos.y + other.pos.y)/2.0);
        self.vel = total_momentum/total_mass;
        self.radius = (((3.0/4.0) * total_vol)/PI).powf(1.0/3.0);
        self.mass = total_mass;
    }
}

impl PartialEq for Planet {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Planet {}

impl fmt::Debug for Planet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Planet: {}", self.id)
    }
}

#[inline]
pub fn newtonian_grav(m1: f64, m2: f64, pos1: &Point2<f64>, pos2: &Point2<f64>) -> Vector2<f64> {
    let dist_vec = Vector2::new(pos2.x - pos1.x, pos2.y - pos1.y);
    let force = (GRAV_CONSTANT * m1 * m2)/(dist_vec.x.powi(2) + dist_vec.y.powi(2));
    let angle = dist_vec.y.atan2(dist_vec.x);

    Vector2::new(force * angle.cos(), force * angle.sin())
}