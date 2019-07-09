use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawParam, Mesh};
use na::Point2;

pub struct MouseInfo {
    pub down: bool,
    pub down_pos: Point2<f32>,
    pub current_drag_position: Point2<f32>,
}

impl MouseInfo {
    pub fn draw_mouse_drag(&self, ctx: &mut Context) -> GameResult {
        let line = Mesh::new_line(
            ctx,
            &[self.down_pos, self.current_drag_position],
            2.0,
            [0.0, 1.0, 0.0, 1.0].into()
        )?;
        graphics::draw(ctx, &line, DrawParam::default())?;

        Ok(())
    }
}

impl Default for MouseInfo {
    fn default() -> MouseInfo {
        MouseInfo {
            down: false,
            down_pos: Point2::new(0.0, 0.0),
            current_drag_position: Point2::new(1.0, 0.0),
        }
    }
}