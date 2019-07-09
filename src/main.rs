extern crate nannou;
extern crate nalgebra as na;
extern crate rand;

mod body;
mod tools;
mod particles;

use crate::body::{Body, BodyType, BodyID};

use na::{
    Point2,
    Vector2,
};
use nannou::{
    prelude::*,
    draw::Draw,
    color,
    time::DurationF64,
};
use std::time::Duration;
use std::collections::HashMap;
use std::cell::RefCell;

pub const GRAV_CONSTANT: f64 = 0.001;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    bodies: HashMap<BodyID, RefCell<Body>>, //Hashmap of ids
    //planet_trail_particlesys: HashMap<BodyID, ParticleSystem>,    // Tied to body id. Seperate from body since i may want effect to last after body is removed.

    collided_planets: Vec<BodyID>, // IDs
    id_counter: BodyID,
}

impl Model {
    fn new() -> Model {
        Model {
            bodies: HashMap::with_capacity(100),
            //planet_trail_particlesys: HashMap::with_capacity(100),
            collided_planets: Vec::with_capacity(20),

            id_counter: 0,
        }
    }

    fn update(&mut self, dt: f64, app_time: &Duration) {
        // self.remove_dead_particle_effects();
        // Remove collided planets
        self.remove_collided_planets();

        let keys: Vec<&u32> = self.bodies.keys().collect();

        for i in 0..keys.len() {   // For each planet
            let mut me = self.bodies.get(keys[i]).unwrap().borrow_mut();
            for j in i+1..keys.len() {  // For every other planet
                let mut other = self.bodies.get(keys[j]).unwrap().borrow_mut();

                if Self::is_colliding(&me.pos, &other.pos, me.radius, other.radius) {
                    //self.bodies.remove(keys[j]);
                    if me.radius < other.radius {
                        other.collide(&me);
                        self.collided_planets.push(*keys[i]);
                    } else {
                        me.collide(&other);
                        self.collided_planets.push(*keys[j]);
                    }
                } else {
                    let df1 = tools::newtonian_grav(
                        me.mass, other.mass,
                        &me.pos, &other.pos
                    );

                    me.res_force += df1;
                    other.res_force -= df1; // Equal and opposite force
                }
            }
            me.update_physics(dt);

            // if planet has trail
            // if let Some(p_trail) = self.planet_trail_particlesys.get_mut(&me.id) {
            //     p_trail.pos = &me.pos;
            // }
        }

        // for (_, sys) in self.planet_trail_particlesys.iter_mut() {
        //     sys.update(dt, app_time);
        // }
    }

    fn display(&self, draw: &Draw, app_time: &Duration) {
        // Display particles behind planets
        // for (_, sys) in self.planet_trail_particlesys.iter() {
        //     sys.display(draw, app_time);
        // }

        for (_, p) in self.bodies.iter() {
            p.borrow().display(draw);
        }
    }

    fn add_body(&mut self, body_type: BodyType, pos: Point2<f64>, vel: Vector2<f64>, radius: f64) {
        self.bodies.insert(self.id_counter, RefCell::new(Body::new(self.id_counter, body_type, pos.clone(), vel, radius, 0.0)));

        // if body_type == BodyType::Planet {
        //     self.planet_trail_particlesys.insert(
        //         self.id_counter,
        //         ParticleSystem::new(pos, ParticleSysType::PlanetTrail)
        //     );
        // }

        self.id_counter = self.id_counter.wrapping_add(1);
    }

    fn remove_collided_planets(&mut self) {
        if self.collided_planets.len() > 0 {
            // Sort out the planet's particle system
            // for key in self.collided_planets.iter() {
            //     if let Some(sys) = self.planet_trail_particlesys.get_mut(key) {
            //         sys.dead = true;
            //     }
            // }

            let temp_c = self.collided_planets.clone();
            self.bodies.retain(|key, _| {
                !temp_c.contains(&key)
            });

            self.collided_planets.clear();
        }
    }

    // #[inline]
    // fn remove_dead_particle_effects(&mut self) {
    //     self.planet_trail_particlesys.retain(|_, sys| !sys.dead || sys.get_particle_count() > 0);
    // }

    fn is_colliding(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        Self::aabb(p1, p2, r1, r2) && tools::distance_squared_to(p1, p2) <= (r1 + r2).powi(2)
    }

    fn aabb(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        let total_rad = r1 + r2;
        p2.x - p1.x <= total_rad && p2.y - p1.y <=  total_rad
    }
}

fn model(_app: &App) -> Model {
    let mut m = Model::new();

    m.add_body(
        BodyType::Planet,
        Point2::new(100.0f64, 100.0),
        Vector2::new(0.0f64, 0.0),
        20.0,
    );
    m.add_body(
        BodyType::Planet,
        Point2::new(20.0f64, 100.0),
        Vector2::new(0.0f64, 0.0),
        30.0,
    );
    m.add_body(
        BodyType::Planet,
        Point2::new(40.0f64, 400.0),
        Vector2::new(0.0f64, 0.0),
        10.0,
    );

    m
}

fn update(app: &App, model: &mut Model, update: Update) {
    let dt: f64 = update.since_last.secs();

    //println!("{}", 1.0/dt);
    model.update(dt, &update.since_start);
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(color::named::BLACK);

    model.display(&draw, &app.duration.since_start);

    draw.to_frame(app, &frame).unwrap();
    frame
}
