use std::{
    process,
    time::{Duration, Instant},
};

pub struct DebugManager {
    pub show_benchmark: bool,
    pub stop_after: usize,
    pub elapsed_blitting: Duration,
    pub elapsed_rendering: Duration,
    pub frame: usize,
}

impl DebugManager {
    pub fn new(show_benchmark: bool, stop_after: usize) -> DebugManager {
        DebugManager {
            show_benchmark,
            stop_after,
            elapsed_blitting: Duration::ZERO,
            elapsed_rendering: Duration::ZERO,
            frame: 0,
        }
    }

    pub fn log_blitting_since(&mut self, time: Instant) {
        self.elapsed_blitting = time.elapsed();
    }

    pub fn log_rendering_since(&mut self, time: Instant) {
        self.elapsed_rendering = time.elapsed();
    }

    pub fn frame(&mut self) {
        if self.stop_after != 0 {
            self.frame += 1;
            if self.frame >= self.stop_after {
                self.print_benchmark(0.0, self.elapsed_blitting + self.elapsed_rendering);
                process::exit(0)
            }
        }
    }

    pub fn print_benchmark(&self, fps: f32, total_elapsed: Duration) {
        if self.show_benchmark {
            println!(
                "Elapsed - Blitting: {:.2?}µs, Printing: {:.2?}µs, Total: {:.2?}µs, Using {:.2?}% of available time",
                self.elapsed_blitting.as_micros(),
                self.elapsed_rendering.as_micros(),
                total_elapsed.as_micros(),
                total_elapsed.as_micros() as f32 / Duration::from_secs_f32(1.0 / fps).as_micros() as f32 * 100.0
            );
        };
    }
}
