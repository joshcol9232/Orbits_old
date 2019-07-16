use ggez::nalgebra as na;
use na::Point2;
use ggez::{GameResult};
use ggez::graphics::spritebatch;

use std::time::Duration;

use crate::particles::{planet_particles::PlanetTrailParticleSys, ParticleSystem};

// const TRAIL_PLACEMENT_PERIOD: f64 = 0.05;
// const TRAIL_NODE_LIFETIME: Duration = Duration::from_millis(1000);
// const TRAIL_NODE_DISTANCE_TOLERANCE: f32 = 2.0; // NOTE: Distance squared

pub struct PlanetTrail {
    pub pos: Point2<f32>, // Not a reference to planet pos, since i want it to live longer than planet
    particles: PlanetTrailParticleSys,
    pub parent_dead: bool,
    // linear_trail: VecDeque<TrailNode>,
    // linear_node_placement_timer: f64,
}

impl PlanetTrail {
    pub fn new(pos: Point2<f32>) -> PlanetTrail {
        let p = PlanetTrail {
            pos,
            particles: PlanetTrailParticleSys::new(),
            parent_dead: false,
            // linear_trail: VecDeque::with_capacity(40),
            // linear_node_placement_timer: 0.0,
        };

        p
    }

    pub fn update(&mut self, dt: f64, current_time: &Duration) {
        // self.kill_dead_nodes(current_time);
        self.particles.update_particles(dt as f32, current_time);

        if !self.parent_dead {
            // Update emmision of particles
            self.particles.update_emmision(dt, current_time, &self.pos);
        }

        //     self.linear_node_placement_timer += dt;
        //     if self.linear_node_placement_timer >= TRAIL_PLACEMENT_PERIOD {
        //         let num = (self.linear_node_placement_timer / TRAIL_PLACEMENT_PERIOD).round();
        //         self.linear_node_placement_timer -= TRAIL_PLACEMENT_PERIOD * num;
        //         self.place_node(current_time);
        //     }
        // }
    }

    #[inline]
    pub fn draw(&self, current_time: &Duration, smoke_sprite_batch: &mut spritebatch::SpriteBatch) -> GameResult {
        self.particles.draw(current_time, smoke_sprite_batch)?;
        // if self.node_count() > 1 {
        //     self.draw_line(ctx, current_time)?;
        // }

        Ok(())
    }

    // fn draw_line(&self, ctx: &mut Context, current_time: &Duration) -> GameResult {
    //     let trail_lifetime_float = timer::duration_to_f64(TRAIL_NODE_LIFETIME);

    //     // Works like a dot-to-dot
    //     for i in 0..self.node_count() - 1 {
    //         let alpha = 1.0
    //             - timer::duration_to_f64(*current_time - self.linear_trail[i].time_created)
    //                 / trail_lifetime_float;
    //         let line = if i == self.node_count() - 2 {
    //             // If on the last line, then connect it to the center of the planet
    //             [self.linear_trail[i].pos, self.pos]
    //         } else {
    //             [self.linear_trail[i].pos, self.linear_trail[i + 1].pos]
    //         };
    //         /* Line colour:
    //             -- Pink 7824e5
    //             -- Blue 23afdd
    //         */
    //         match Mesh::new_line(
    //             ctx,
    //             &line,
    //             2.0,
    //             [0.13671875, 0.68359375, 0.86328125, alpha as f32].into(),
    //         ) {
    //             Ok(line_mesh) => {
    //                 graphics::draw(ctx, &line_mesh, DrawParam::default())?;
    //             }
    //             Err(e) => {} // eprintln!("Issue drawing line in planet trail. {}", e); }
    //         }
    //     }

    //     Ok(())
    // }

    // fn place_node(&mut self, current_time: &Duration) {
    //     // Makes sure node cannot be placed too close to last one as to cause a drawing error
    //     let can_place = if self.node_count() > 1 {
    //         tools::distance_squared_to(&self.linear_trail[self.node_count() - 1].pos, &self.pos)
    //             > TRAIL_NODE_DISTANCE_TOLERANCE
    //     } else {
    //         true
    //     };

    //     if can_place {
    //         self.linear_trail.push_back(TrailNode {
    //             pos: self.pos,
    //             time_created: current_time.clone(),
    //         });
    //     }
    // }

    // #[inline]
    // fn kill_dead_nodes(&mut self, current_time: &Duration) {
    //     kill_objects_with_lifetime!(self.linear_trail, current_time, TRAIL_NODE_LIFETIME);
    // }

    #[inline]
    pub fn particle_count(&self) -> usize {
        self.particles.particle_count()
    }

    #[inline]
    pub fn node_count(&self) -> usize {
        0 // self.linear_trail.len()
    }
}

// #[derive(Debug)]
// struct TrailNode {
//     pos: Point2<f32>,
//     time_created: Duration,
// }