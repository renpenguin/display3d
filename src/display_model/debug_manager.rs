use std::time::{Duration, Instant};

pub struct DebugManager {
    show_benchmark: bool,
    elapsed_blitting: Duration,
    elapsed_rendering: Duration,
}

impl DebugManager {
    #[must_use]
    pub const fn new(show_benchmark: bool) -> Self {
        Self {
            show_benchmark,
            elapsed_blitting: Duration::ZERO,
            elapsed_rendering: Duration::ZERO,
        }
    }

    pub fn log_blitting_since(&mut self, time: Instant) {
        self.elapsed_blitting = time.elapsed();
    }

    pub fn log_rendering_since(&mut self, time: Instant) {
        self.elapsed_rendering = time.elapsed();
    }

    pub fn print_benchmark(&self, fps: f32, total_elapsed: Duration) {
        if self.show_benchmark {
            println!(
                "Elapsed - Blitting: {:>5}µs, Printing: {:>5}µs, Total: {:>5}µs, Using {:>5.2?}% of available time per frame",
                self.elapsed_blitting.as_micros(),
                self.elapsed_rendering.as_micros(),
                total_elapsed.as_micros(),
                total_elapsed.as_micros() as f32 / Duration::from_secs_f32(1.0 / fps).as_micros() as f32 * 100.0
            );
        };
    }
}
