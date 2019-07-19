use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, DrawMode, DrawParam};

use euclid::Point2D;


pub fn draw_planet(ctx: &mut Context, pos: &Point2D<f64, f64>, radius: f64) -> GameResult {
    let planet_mesh = Mesh::new_circle(
        ctx,
        DrawMode::fill(),
        [0.0, 0.0],
        radius as f32,
        0.1,
        [0.9, 0.9, 0.9, 1.0].into()
    )?;

    graphics::draw(ctx, &planet_mesh,
        DrawParam::default()
            .dest([pos.x as f32, pos.y as f32])
    )
}