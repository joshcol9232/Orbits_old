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
        let mut cutoff_index = 0usize;
        for (i, n) in $queue.iter().enumerate() {
            if *$time - n.time_created > $max_lifetime {
                cutoff_index = i;
            } else { break }
        }

        for _ in 0..cutoff_index {
            $queue.pop_front();
        }
    };
}