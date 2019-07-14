#[macro_use]
mod macros;

mod mouse;
mod particles;
mod planet;
mod tools;

use ggez::{
    event::{self, KeyCode, KeyMods, MouseButton},
    graphics::{self, DrawMode, DrawParam, Mesh},
    nalgebra as na, timer, Context, GameResult,
    filesystem,
};
use na::{Point2, RealField, Vector2};
use serde::{Serialize, Deserialize};

use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::io::{Read, Write};

use crate::{
    mouse::MouseInfo,
    planet::{Planet, PlanetSaveData, PlanetID, PlanetTrail},
};

pub const TWO_PI: f64 = std::f64::consts::PI * 2.0;
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
    smoke_sprite_batch: graphics::spritebatch::SpriteBatch,

    planets: HashMap<PlanetID, RefCell<Planet>>,    // Hashmap of ids
    planet_trails: HashMap<PlanetID, PlanetTrail>,  // Tied to body id. Seperate from body since i may want effect to last after body is removed.

    collided_planets: Vec<PlanetID>, // IDs
    id_counter: PlanetID,

    mouse_info: MouseInfo,

    temp_save: Option<SaveState>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let smoke_image = graphics::Image::new(ctx, "/smokeparticle.png").unwrap();

        let mut s = MainState {
            smoke_sprite_batch: graphics::spritebatch::SpriteBatch::new(smoke_image),

            planets: HashMap::with_capacity(100),
            planet_trails: HashMap::with_capacity(100),
            collided_planets: Vec::with_capacity(20),

            id_counter: 0,

            mouse_info: MouseInfo::default(),

            temp_save: None,
        };

        s.add_planet(
            ctx,
            Point2::new(500.0f64, 400.0),
            Vector2::new(0.0f64, 0.0),
            50.0
        );

        //s.spawn_square_of_planets(ctx, Point2::new(50.0, 50.0), 20, 20, 50.0, 5.0);

        Ok(s)
    }

    fn add_planet(&mut self, ctx: &Context, pos: Point2<f64>, vel: Vector2<f64>, radius: f64) {
        self.planets.insert(
            self.id_counter,
            RefCell::new(Planet::new(self.id_counter, pos.clone(), vel, radius, 0.0)),
        );

        self.add_planet_trail(self.id_counter, cast_point2_to_f32!(pos));

        self.id_counter = self.id_counter.wrapping_add(1);
    }

    fn add_planet_trail(&mut self, id: PlanetID, pos: Point2<f32>) {
        self.planet_trails.insert(
            id,
            PlanetTrail::new(pos),
        );
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
            self.planets.retain(|key, _| !temp_c.contains(&key));

            self.collided_planets.clear();
        }
    }

    #[inline]
    fn remove_dead_planet_trails(&mut self) {
        // > 1 nodes needed to draw a line
        self.planet_trails
            .retain(|_, sys| !sys.parent_dead || sys.particle_count() > 0 || sys.node_count() > 1);
    }

    fn is_colliding(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        Self::aabb(p1, p2, r1, r2) && tools::distance_squared_to(p1, p2) <= (r1 + r2).powi(2)
    }

    fn aabb(p1: &Point2<f64>, p2: &Point2<f64>, r1: f64, r2: f64) -> bool {
        let total_rad = r1 + r2;
        p2.x - p1.x <= total_rad && p2.y - p1.y <= total_rad
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
            [0.4, 0.4, 0.4, 1.0].into(),
        )?;

        graphics::draw(ctx, &circ, DrawParam::default().dest(pos))?;

        Ok(())
    }

    fn draw_fps_and_info(&self, ctx: &mut Context) -> GameResult {
        use graphics::Text;
        let text = Text::new(format!("{:.2}\nPlanets: {}\nParticles: {}", timer::fps(ctx), self.planets.len(), self.get_total_particle_count()));

        graphics::draw(
            ctx,
            &text,
            DrawParam::default().dest(Point2::new(10.0, 10.0)),
        )?;
        Ok(())
    }

    fn spawn_square_of_planets(
        &mut self,
        ctx: &Context,
        top_left: Point2<f64>,
        w: u16,
        h: u16,
        gap: f64,
        rad: f64,
    ) {
        for i in 0..w {
            for j in 0..h {
                self.add_planet(
                    ctx,
                    Point2::new(top_left.x + i as f64 * gap, top_left.y + j as f64 * gap),
                    Vector2::new(0.0, 0.0),
                    rad,
                );
            }
        }
    }

    #[inline]
    fn clear_planets(&mut self) {
        self.planets.clear();
        for (_, sys) in self.planet_trails.iter_mut() {
            sys.parent_dead = true;
        }
        self.collided_planets.clear();
        self.id_counter = 0;
    }

    #[inline]
    fn clear_trails(&mut self) {
        self.planet_trails.clear();
    }

    #[inline]
    fn clear_planets_and_trails(&mut self) {
        self.planet_trails.clear();
        self.planets.clear();
        self.collided_planets.clear();
        self.id_counter = 0;
    }
    
    #[inline]
    fn clear_all(&mut self) {
        self.clear_trails();
        self.clear_planets();
    }

    fn save_to_file(&self, ctx: &mut Context, path: &Path) -> GameResult {
        println!("Saving: {}", path.display());
        let save = SaveState::new_from_main_state(&self);
        let encoded = bincode::serialize(&save).unwrap();

        let mut file = filesystem::create(ctx, path)?;
        file.write(encoded.as_slice())?;

        Ok(())
    }

    fn load_from_file(&mut self, ctx: &mut Context, path: &Path) -> GameResult {
        println!("Loading: {}", path.display());
        let save_state = SaveState::load_from_file(ctx, path)?;
        self.load_from_save_state(&save_state);
        Ok(())
    }

    fn save_to_temp_save(&mut self, ctx: &mut Context) {
        println!("Saving to temporary save.");
        self.temp_save = Some(SaveState::new_from_main_state(&self));
    }

    fn load_from_temp_save(&mut self) {
        let temp = self.temp_save.take();
        match temp {
            None => self.clear_all(),
            Some(ref temp_save) => {
                println!("Loading from temporary save.");
                self.load_from_save_state(temp_save);
            }
        }
        self.temp_save = temp;
    }

    fn load_planet(&mut self, saved_planet: &PlanetSaveData) {
        let mut loaded_planet: Planet = saved_planet.into();
        let new_id = self.id_counter;
        self.id_counter += 1;
        loaded_planet.id = new_id;

        self.planets.insert(new_id, RefCell::new(loaded_planet));
        self.add_planet_trail(new_id, Point2::new(saved_planet.pos_x as f32, saved_planet.pos_y as f32));
    }

    #[inline]
    fn load_from_save_state(&mut self, save: &SaveState) {
        self.clear_all();
        self.load_planets_from_save_state(save);
    }

    #[inline]
    fn load_planets_from_save_state(&mut self, save: &SaveState) {
        for (_, saved_planet) in save.planets.iter() {
            self.load_planet(saved_planet);
        }
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

        for i in 0..keys.len() {
            // For each planet
            let mut me = self.planets.get(keys[i]).unwrap().borrow_mut();
            for j in i + 1..keys.len() {
                // For every other planet
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
                    let df1 = tools::newtonian_grav(me.mass, other.mass, &me.pos, &other.pos);

                    me.res_force += df1;
                    other.res_force -= df1; // Equal and opposite force
                }
            }
            me.update_physics(dt);

            // if planet has trail
            if let Some(p_trail) = self.planet_trails.get_mut(&me.id) {
                p_trail.pos.x = me.pos.x as f32;
                p_trail.pos.y = me.pos.y as f32;
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
            sys.draw(ctx, &time_since_start, &mut self.smoke_sprite_batch)?;
        }
        graphics::draw(ctx, &self.smoke_sprite_batch, DrawParam::new())?;
        self.smoke_sprite_batch.clear();

        for (_, rc) in self.planets.iter() {
            rc.borrow().draw(ctx)?;
        }

        if self.mouse_info.down
            && self.mouse_info.button_down == MouseButton::Left
            && tools::distance_squared_to(
                &self.mouse_info.down_pos,
                &self.mouse_info.current_drag_position,
            ) >= 4.0
        {
            self.mouse_info.draw_mouse_drag(ctx)?;
            self.draw_fake_planet(ctx, self.mouse_info.down_pos, 5.0)?;
        }

        self.draw_fps_and_info(ctx)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_info.down = true;
        self.mouse_info.button_down = button;
        self.mouse_info.down_pos = Point2::new(x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.mouse_info.down = false;
        let origin = Point2::new(
            self.mouse_info.down_pos.x as f64,
            self.mouse_info.down_pos.y as f64,
        );

        if button == MouseButton::Left {
            self.add_planet(ctx, origin, origin - Point2::new(x as f64, y as f64), 5.0);
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse_info.current_drag_position = Point2::new(x, y);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key: KeyCode,
        mods: KeyMods,
        _repeat: bool,
    ) {
        match key {
            KeyCode::R => {
                self.clear_planets();
                if mods.contains(KeyMods::CTRL) {       // CTRL + R to clear planets AND save
                    self.temp_save = None;
                }
            },
            KeyCode::S => {
                if mods.contains(KeyMods::CTRL) {
                    self.save_to_file(ctx, Path::new("/save.bin")).unwrap();
                } else {
                    self.save_to_temp_save(ctx);
                }
            },
            KeyCode::L => {
                if mods.contains(KeyMods::CTRL) {
                    self.load_from_file(ctx, Path::new("/save.bin")).unwrap();
                } else {
                    self.load_from_temp_save();
                }
            },
            _ => ()
        }
    }
}

// Important fields from MainState
#[derive(Serialize, Deserialize, Default)]
struct SaveState {
    planets: HashMap<PlanetID, PlanetSaveData>,
}

impl SaveState {
    fn new_from_main_state(main: &MainState) -> SaveState {
        SaveState {
            planets: Self::planet_save_data_from_planets(&main.planets),
        }
    }

    fn load_from_file(ctx: &mut Context, path: &Path) -> GameResult<SaveState> {
        let mut file = filesystem::open(ctx, path)?;
        let mut full_data = Vec::<u8>::new();
        file.read_to_end(&mut full_data)?;

        let mut save = SaveState::default();
        save.planets = bincode::deserialize(full_data.as_slice()).unwrap();

        Ok(save)
    }

    fn planet_save_data_from_planets(map: &HashMap<PlanetID, RefCell<Planet>>) -> HashMap<PlanetID, PlanetSaveData> {
        let mut out_map: HashMap<PlanetID, PlanetSaveData> = HashMap::new();
        for (key, val) in map.iter() {
            out_map.insert(*key, val.borrow().into());
        }
        out_map
    }
}


pub fn main() -> GameResult {
    use ggez::conf::{NumSamples, WindowSetup, WindowMode, FullscreenType};

    let mut cb = ggez::ContextBuilder::new("Orbits", "eggmund")
        .window_setup(WindowSetup {
            title: "Orbits".to_owned(),
            samples: NumSamples::Eight,
            vsync: true,
            transparent: false,
            icon: "".to_owned(),
            srgb: true
        })
        .window_mode(WindowMode {
            width: 1000.0,
            height: 800.0,
            maximized: false,
            fullscreen_type: FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            min_height: 0.0,
            max_width: 0.0,
            max_height: 0.0,
            hidpi: false,
            resizable: false
        });

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
        cb = cb.add_resource_path(path);
    }

    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
