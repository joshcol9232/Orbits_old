extern crate ggez;
extern crate rand;

#[macro_use] mod macros;

mod planet;
mod tools;
mod particles;
mod mouse;

use ggez::{
    Context, GameResult,
    timer,
    nalgebra as na,
    graphics::{self, DrawParam, DrawMode, Mesh},
    event::{self, MouseButton},
};
use na::{
    Point2,
    Vector2,
    RealField,
};

use std::collections::HashMap;
use std::cell::RefCell;

use crate::{
    planet::{Planet, PlanetID, PlanetTrail},
    mouse::MouseInfo,
};


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
    planet_trails: HashMap<PlanetID, PlanetTrail>,    // Tied to body id. Seperate from body since i may want effect to last after body is removed.

    collided_planets: Vec<PlanetID>, // IDs
    id_counter: PlanetID,

    mouse_info: MouseInfo,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut s = MainState {
            planets: HashMap::with_capacity(100),
            planet_trails: HashMap::with_capacity(100),
            collided_planets: Vec::with_capacity(20),

            id_counter: 0,

            mouse_info: MouseInfo::default(),
        };

        s.add_planet(
            Point2::new(200.0f64, 100.0),
            Vector2::new(0.0f64, 0.0),
            20.0,
            ctx
        );
        s.add_planet(
            Point2::new(300.0f64, 300.0),
            Vector2::new(0.0f64, 0.0),
            30.0,
            ctx
        );
        s.add_planet(
            Point2::new(40.0f64, 400.0),
            Vector2::new(0.0f64, 0.0),
            10.0,
            ctx
        );

        Ok(s)
    }

    fn add_planet(&mut self, pos: Point2<f64>, vel: Vector2<f64>, radius: f64, ctx: &Context) {
        self.planets.insert(self.id_counter, RefCell::new(Planet::new(self.id_counter, pos.clone(), vel, radius, 0.0)));

        self.planet_trails.insert(
            self.id_counter,
            PlanetTrail::new(pos, &timer::time_since_start(ctx)),
        );

        self.id_counter = self.id_counter.wrapping_add(1);
    }

    fn remove_collided_planets(&mut self) {
        if self.collided_planets.len() > 0 {
            // Sort out the planet's particle system
            for key in self.collided_planets.iter() {
                if let Some(sys) = self.planet_trails.get_mut(key) {
                    // If the planet no longer exists, then set the particle system to dead. 
                    // If the particle system is dead, it will no longer emit, but will be removed when
                    // all nodes/particles have faded (see `remove_dead_planet_trails`).
                    sys.parent_dead = true;
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
    fn remove_dead_planet_trails(&mut self) {
        // > 1 nodes needed to draw a line
        self.planet_trails.retain(|_, sys| !sys.parent_dead || sys.particle_count() > 0 || sys.node_count() > 1);
    }

    fn is_colliding(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        Self::aabb(p1, p2, r1, r2) && tools::distance_squared_to(p1, p2) <= (r1 + r2).powi(2)
    }

    fn aabb(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        let total_rad = r1 + r2;
        p2.x - p1.x <= total_rad && p2.y - p1.y <=  total_rad
    }

    fn get_total_particle_count(&self) -> usize {
        let mut count = 0;
        for (_, p) in self.planet_trails.iter() {
            count += p.particle_count();
        }
        count
    }

    #[inline]
    fn draw_fake_planet(&self, ctx: &mut Context, pos: Point2<f32>, rad: f32) -> GameResult {
        let circ = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            Point2::new(0.0, 0.0),
            rad,
            0.05,
            [0.4, 0.4, 0.4, 1.0].into()
        )?;

        graphics::draw(
            ctx,
            &circ,
            DrawParam::default().dest(pos)
        )?;

        Ok(())
    }

    fn draw_fps(ctx: &mut Context) -> GameResult {
        use graphics::Text;
        let text = Text::new(format!("{:.2}", timer::fps(ctx)));

        graphics::draw(ctx, &text, DrawParam::default().dest(Point2::new(10.0, 10.0)))?;
        Ok(())
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx));
        let time_since_start = timer::time_since_start(ctx);

        //println!("Particles: {}", self.get_total_particle_count());

        self.remove_dead_planet_trails();
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
            if let Some(p_trail) = self.planet_trails.get_mut(&me.id) {
                p_trail.pos = me.pos;
            }
        }

        for (_, trail_sys) in self.planet_trails.iter_mut() {
            trail_sys.update(dt, &time_since_start);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let time_since_start = timer::time_since_start(ctx);
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        // Display particles behind planets
        for (_, sys) in self.planet_trails.iter() {
            sys.draw(ctx, &time_since_start)?;
        }

        for (_, rc) in self.planets.iter() {
            rc.borrow().draw(ctx)?;
        }

        if self.mouse_info.down && tools::distance_squared_to(&self.mouse_info.down_pos, &self.mouse_info.current_drag_position) >= 4.0 {
            self.mouse_info.draw_mouse_drag(ctx)?;
            self.draw_fake_planet(ctx, self.mouse_info.down_pos, 5.0)?;
        }

        Self::draw_fps(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.mouse_info.down = true;
        self.mouse_info.down_pos = Point2::new(x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.mouse_info.down = false;
        let origin = Point2::new(self.mouse_info.down_pos.x as f64, self.mouse_info.down_pos.y as f64);

        self.add_planet(origin, origin - Point2::new(x as f64, y as f64), 5.0, ctx);
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        // Set pos
        self.mouse_info.current_drag_position = Point2::new(x, y);
    }
}

pub fn main() -> GameResult {
    use ggez::conf::{WindowSetup, NumSamples};

    let cb = ggez::ContextBuilder::new("Orbits", "eggmund")
        .window_setup(WindowSetup {
            title: "Orbits".to_owned(),
            samples: NumSamples::Eight,
            vsync: true,
            transparent: false,
            icon: "".to_owned(),
            srgb: true,
        });

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}