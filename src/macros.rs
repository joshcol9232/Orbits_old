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

#[macro_export]
macro_rules! particle_system_defaults {
    ($max_lifetime:expr) => {
        #[inline]
        fn kill_particles(&mut self, current_time: &Duration) {
            kill_objects_with_lifetime!(self.particles, current_time, $max_lifetime);
        }
        #[inline]
        fn particle_count(&self) -> usize { self.particles.len() }
    };
}

#[macro_export]
macro_rules! particle_set_get_defaults {
    // $lifetime is &Duration
    ($lifetime:expr) => {
        #[inline]
        fn time_created(&self) -> &std::time::Duration { &self.time_created }
        #[inline]
        fn lifetime(&self) -> &std::time::Duration { $lifetime }
        #[inline]
        fn rad(&self) -> f32 { self.rad }
    };
}

// Macro for a common pattern, which is when you have a list of objects with a lifetime that need to be
// removed from a queue.
// Queue is in order of when the element was added, so nodes to remove
// will be at the start of the queue. I just need to know how many (if any)
// have to be removed from the start.
// Super useful macro.
#[macro_export]
macro_rules! kill_objects_with_lifetime {
    // $queue is a VecDeque, $time is &Duration, $max_lifetime is Duration
    ($queue:expr, $time:expr, $max_lifetime:expr) => {
        while $queue.len() > 0 && *$time - $queue.front().unwrap().time_created > $max_lifetime {
            $queue.pop_front();
        }
    };
}

#[macro_export]
macro_rules! cast_point2_to_f32 {
    ($point:expr) => {
        Point2::new($point.x as f32, $point.y as f32)
    };
}