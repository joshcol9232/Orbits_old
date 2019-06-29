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
};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    planets: Vec<Planet>,
    collided_planets: Vec<u32>, // IDs
    id_counter: u32,
}

impl Model {
    fn new() -> Model {
        Model {
            planets: vec![],
            collided_planets: vec![],

            id_counter: 0,
        }
    }

    fn update(&mut self, dt: f64) {
        // Remove collided planets
        self.remove_collided_planets();

        for i in 0..self.planets.len() {   // For each planet
            if !self.collided_planets.contains(&self.planets[i].id) {
                for j in i+1..self.planets.len() {  // For every other planet
                    if !self.collided_planets.contains(&self.planets[j].id) {
                        if Self::is_colliding(&self.planets[i].pos, &self.planets[j].pos, self.planets[i].radius, self.planets[j].radius) {
                            let tmp = self.planets[j].clone();
                            self.planets[i] += &tmp;
                            self.collided_planets.push(self.planets[j].id);
                        } else {
                            let df1 = planet::newtonian_grav(
                                self.planets[i].mass, self.planets[j].mass,
                                &self.planets[i].pos, &self.planets[j].pos
                            );

                            self.planets[i].res_force += df1;
                            self.planets[j].res_force -= df1; // Equal and opposite force
                        }
                    }
                }
                self.planets[i].update(dt);
            }
        }
    }

    fn display(&self, draw: &Draw) {
        for p in self.planets.iter() {
            p.display(draw);
        }
    }

    fn add_planet(&mut self, pos: Point2<f64>, vel: Vector2<f64>, radius: f64) {
        self.planets.push(Planet::new(self.id_counter, pos, vel, radius, 0.0));

        if self.id_counter >= std::u32::MAX {
            self.id_counter = 0
        } else {
            self.id_counter += 1;
        }
    }

    fn remove_collided_planets(&mut self) {
        if self.collided_planets.len() > 0 {
            let temp_c = self.collided_planets.clone();
            self.planets.retain(|pl| {
                !temp_c.contains(&pl.id)
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
        10.0,
    );
    m.add_planet(
        Point2::new(20.0f64, 100.0),
        Vector2::new(0.0f64, 0.0),
        10.0,
    );

    m
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let dt = update.since_last.as_nanos() as f64 * 0.000000001;
    model.update(dt);
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(color::named::BLACK);

    model.display(&draw);

    draw.to_frame(app, &frame).unwrap();
    frame
}
