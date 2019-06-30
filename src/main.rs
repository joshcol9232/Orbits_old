extern crate nannou;
extern crate nalgebra as na;

mod planet;
mod tools;

use crate::planet::Planet;

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

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    planets: HashMap<u32, RefCell<Planet>>, //Hashmap of ids  Vec<Planet>,
    collided_planets: Vec<u32>, // IDs
    id_counter: u32,
}

impl Model {
    fn new() -> Model {
        Model {
            planets: HashMap::new(),
            collided_planets: vec![],

            id_counter: 0,
        }
    }

    fn update(&mut self, dt: f64, app_time: &Duration) {
        // Remove collided planets
        self.remove_collided_planets();

        let mut keys: Vec<&u32> = self.planets.keys().collect();

        for i in 0..keys.len() {   // For each planet
            let mut me = self.planets.get(keys[i]).unwrap().borrow_mut();
            for j in i+1..keys.len() {  // For every other planet
                let mut other = self.planets.get(keys[j]).unwrap().borrow_mut();

                if Self::is_colliding(&me.pos, &other.pos, me.radius, other.radius) {
                    //self.planets.remove(keys[j]);
                    if me.radius < other.radius {
                        other.collide(&me);
                        self.collided_planets.push(*keys[i]);
                    } else {
                        me.collide(&other);
                        self.collided_planets.push(*keys[j]);
                    }
                } else {
                    let df1 = planet::newtonian_grav(
                        me.mass, other.mass,
                        &me.pos, &other.pos
                    );

                    me.res_force += df1;
                    other.res_force -= df1; // Equal and opposite force
                }
            }
            me.update(dt, app_time);
        }
    }

    fn display(&self, draw: &Draw) {
        for (_, p) in self.planets.iter() {
            p.borrow().display(draw);
        }
    }

    fn add_planet(&mut self, pos: Point2<f64>, vel: Vector2<f64>, radius: f64) {
        self.planets.insert(self.id_counter, RefCell::new(Planet::new(self.id_counter, pos, vel, radius, 0.0)));

        self.id_counter = self.id_counter.wrapping_add(1);
    }

    fn remove_collided_planets(&mut self) {
        if self.collided_planets.len() > 0 {
            let temp_c = self.collided_planets.clone();
            self.planets.retain(|key, _| {
                !temp_c.contains(&key)
            });

            self.collided_planets = vec![];
        }
    }

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

    m.add_planet(
        Point2::new(100.0f64, 100.0),
        Vector2::new(0.0f64, 0.0),
        20.0,
    );
    m.add_planet(
        Point2::new(20.0f64, 100.0),
        Vector2::new(0.0f64, 0.0),
        30.0,
    );
    m.add_planet(
        Point2::new(40.0f64, 400.0),
        Vector2::new(0.0f64, 0.0),
        10.0,
    );

    m
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt: f64 = update.since_last.secs();
    model.update(dt, &update.since_start);
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(color::named::BLACK);

    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
    frame
}
