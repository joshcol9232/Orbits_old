#[macro_use]
extern crate specs_derive;

mod components;
mod systems;
mod render;
mod time;
mod tools;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use euclid::{Point2D, Vector2D};

use specs::prelude::*;

use crate::{
    components::{
        Pos, Vel, Force, Mass,
        shape::Shape,
        render::PlanetRender,
    },
    systems::{
        gravity::GravitySys,
        motion::MotionSys,
        collision::CollisionSys,
    }
};

const PLANET_DENSITY: f64 = 1000.0;

struct MainState {
    world: World,
    dispatcher: specs::Dispatcher<'static, 'static>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut ecs_world = World::new();

        let mut dispatcher = Self::register_systems();
        Self::register_misc_components(&mut ecs_world);
        dispatcher.setup(&mut ecs_world);

        ecs_world.insert(time::DeltaTime(1.0/60.0));

        Self::add_planet(
            &mut ecs_world,
            Point2D::new(400.0, 300.0),
            Vector2D::new(0.0, 0.0),
            5.0
        );
        Self::add_planet(
            &mut ecs_world,
            Point2D::new(200.0, 100.0),
            Vector2D::new(0.0, 0.0),
            50.0
        );
        Self::add_planet(
            &mut ecs_world,
            Point2D::new(800.0, 100.0),
            Vector2D::new(0.0, 0.0),
            10.0
        );


        dispatcher.dispatch(&ecs_world);
        ecs_world.maintain();

        let s = MainState {
            world: ecs_world,
            dispatcher,
        };

        Ok(s)
    }

    fn register_systems() -> specs::Dispatcher<'static, 'static> {
        DispatcherBuilder::new()
            .with(MotionSys, "motion_sys", &[])
            .with(CollisionSys, "collision_sys", &["motion_sys"])
            .with(GravitySys::default(), "gravity_sys", &["motion_sys"])
            .build()
    }

    fn register_misc_components(world: &mut World) {
        world.register::<Shape>();
        world.register::<PlanetRender>();
    }

    fn add_planet(world: &mut World, pos: Point2D<f64, f64>, vel: Vector2D<f64, f64>, radius: f64) {
        world.create_entity()
            .with(Pos(pos))
            .with(Vel(vel))
            .with(Mass(Self::get_new_planet_mass(radius)))
            .with(Force(Vector2D::new(0.0, 0.0)))
            .with(Shape::Circle(radius))
            .with(PlanetRender)
            .build();
    }

    #[inline]
    fn get_new_planet_mass(radius: f64) -> f64 {
        tools::sphere_volume(radius) * PLANET_DENSITY
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update dt:
        {
            let mut delta = self.world.write_resource::<time::DeltaTime>();
            delta.0 = ggez::timer::duration_to_f64(ggez::timer::delta(ctx));
        }

        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let pos = self.world.read_storage::<Pos>();
        let is_planet = self.world.read_storage::<PlanetRender>();
        let shape = self.world.read_storage::<Shape>();

        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for (pos, _, shape) in (&pos, &is_planet, &shape).join() {
            if let Shape::Circle(radius) = shape {
                render::draw_planet(ctx, &pos.0, *radius)?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
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