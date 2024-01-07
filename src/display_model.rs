use std::time::{Duration, Instant};

use gemini_engine::{
    elements::view::{ScaleFitView, Wrapping},
    elements3d::{DisplayMode, Mesh3D, Transform3D, Viewport},
    gameloop::{sleep_fps, MainLoopRoot},
};

#[allow(dead_code)]
pub struct Root {
    canvas: ScaleFitView,
    viewport: Viewport,
    display_mode: DisplayMode,
    models: Vec<Mesh3D>,
    // Timing stats
    show_benchmark: bool,
    elapsed_blitting: Duration,
    elapsed_rendering: Duration,
}

impl Root {
    pub fn new(
        canvas: ScaleFitView,
        fov: f64,
        initial_viewport_transform: Transform3D,
        models: Vec<Mesh3D>,
        display_mode: DisplayMode,
        show_benchmark: bool,
    ) -> Root {
        let viewport_center = canvas.intended_size() / 2;
        Root {
            canvas,
            viewport: Viewport::new(initial_viewport_transform, fov, viewport_center),
            display_mode,
            models,
            show_benchmark,
            elapsed_blitting: Duration::ZERO,
            elapsed_rendering: Duration::ZERO,
        }
    }
}

impl MainLoopRoot for Root {
    type InputDataType = u8;
    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        self.viewport.transform.rotation.y += 0.05;
    }

    fn render_frame(&mut self) {
        // Auto-resize
        self.viewport.origin = self.canvas.intended_size() / 2;
        self.canvas.update();

        let now = Instant::now();

        // let objects: Vec<&dyn ViewElement3D> = ;
        self.canvas.view.blit(
            &self.viewport.render(
                self.models.iter().map(|m| m as _).collect(),
                self.display_mode.clone(),
            ),
            Wrapping::Ignore,
        );

        self.elapsed_blitting = now.elapsed();

        let now = Instant::now();
        self.canvas.view.display_render().unwrap();
        self.elapsed_rendering = now.elapsed();
    }

    fn sleep_and_get_input_data(
        &self,
        fps: f32,
        elapsed: Duration,
    ) -> (bool, Option<Self::InputDataType>) {
        // Hijack the sleep function to print elapsed times before falling back to default sleep function
        if self.show_benchmark {
            println!(
                "Elapsed - Blitting: {:.2?}µs, Printing: {:.2?}µs, Total: {:.2?}µs, Using {:.2?}% of available time",
                self.elapsed_blitting.as_micros(),
                self.elapsed_rendering.as_micros(),
                elapsed.as_micros(),
                elapsed.as_micros() as f32 / Duration::from_secs_f32(1.0 / fps).as_micros() as f32 * 100.0
            );
        };

        (sleep_fps(fps, Some(elapsed)), None)
    }
}
