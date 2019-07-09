extern crate ggez;
extern crate rand;

#[macro_export]
macro_rules! mobile_get_set_defaults {
    ($t:ty) => {
        #[inline(always)]
        fn pos(&self) -> &Point2<$t> { &self.pos }
        #[inline(always)]
        fn pos_mut(&mut self) -> &mut Point2<$t> { &mut self.pos }
        #[inline(always)]
        fn vel(&self) -> &Vector2<$t> { &self.vel }
        #[inline(always)]
        fn vel_mut(&mut self) -> &mut Vector2<$t> { &mut self.vel }
    };
}

mod planet;
mod tools;
mod particles;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::timer;

use crate::planet::{Planet, PlanetID};
use crate::particles::planet_trail::PlanetTrailParticleSys;
use crate::particles::ParticleSystem;

use na::{
    Point2,
    Vector2,
    RealField,
};
use std::collections::HashMap;
use std::cell::RefCell;

pub const GRAV_CONSTANT: f64 = 0.001;


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

struct MainState {
    planets: HashMap<PlanetID, RefCell<Planet>>, //Hashmap of ids
    planet_trail_particlesys: HashMap<PlanetID, PlanetTrailParticleSys>,    // Tied to body id. Seperate from body since i may want effect to last after body is removed.

    collided_planets: Vec<PlanetID>, // IDs
    id_counter: PlanetID,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            planets: HashMap::with_capacity(100),
            planet_trail_particlesys: HashMap::with_capacity(100),
            collided_planets: Vec::with_capacity(20),

            id_counter: 0,
        };

        s.add_planet(
            Point2::new(200.0f64, 100.0),
            Vector2::new(0.0f64, 0.0),
            20.0,
        );
        s.add_planet(
            Point2::new(300.0f64, 300.0),
            Vector2::new(0.0f64, 0.0),
            30.0,
        );
        s.add_planet(
            Point2::new(40.0f64, 400.0),
            Vector2::new(0.0f64, 0.0),
            10.0,
        );

        Ok(s)
    }

    fn get_total_particle_count(&self) -> usize {
        let mut count = 0;
        for (_, p) in self.planet_trail_particlesys.iter() {
            count += p.particle_count();
        }
        count
    }

    fn add_planet(&mut self, pos: Point2<f64>, vel: Vector2<f64>, radius: f64) {
        self.planets.insert(self.id_counter, RefCell::new(Planet::new(self.id_counter, pos.clone(), vel, radius, 0.0)));

        self.planet_trail_particlesys.insert(
            self.id_counter,
            PlanetTrailParticleSys::new(pos)
        );

        self.id_counter = self.id_counter.wrapping_add(1);
    }

    fn remove_collided_planets(&mut self) {
        if self.collided_planets.len() > 0 {
            // Sort out the planet's particle system
            for key in self.collided_planets.iter() {
                if let Some(sys) = self.planet_trail_particlesys.get_mut(key) {
                    sys.dead = true;
                }
            }

            let temp_c = self.collided_planets.clone();
            self.planets.retain(|key, _| {
                !temp_c.contains(&key)
            });

            self.collided_planets.clear();
        }
    }

    #[inline]
    fn remove_dead_particle_effects(&mut self) {
        self.planet_trail_particlesys.retain(|_, sys| !sys.dead || sys.get_particle_count() > 0);
    }

    fn is_colliding(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        Self::aabb(p1, p2, r1, r2) && tools::distance_squared_to(p1, p2) <= (r1 + r2).powi(2)
    }

    fn aabb(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        let total_rad = r1 + r2;
        p2.x - p1.x <= total_rad && p2.y - p1.y <=  total_rad
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx));
        let time_since_start = timer::time_since_start(ctx);

        //println!("Particles: {}", self.get_total_particle_count());

        self.remove_dead_particle_effects();
        self.remove_collided_planets();

        let keys: Vec<&u32> = self.planets.keys().collect();

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
            if let Some(p_trail) = self.planet_trail_particlesys.get_mut(&me.id) {
                p_trail.pos = me.pos;
            }
        }

        for (_, sys) in self.planet_trail_particlesys.iter_mut() {
            sys.update(dt, &time_since_start);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let time_since_start = timer::time_since_start(ctx);
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Display particles behind planets
        for (_, sys) in self.planet_trail_particlesys.iter() {
            sys.draw(ctx, &time_since_start)?;
        }

        for (_, rc) in self.planets.iter() {
            rc.borrow().draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}