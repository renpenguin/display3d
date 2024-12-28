use std::time::{Duration, Instant};

use gemini_engine::{
    elements::{
        containers::CanShade,
        view::{ScaleFitView, Wrapping},
    },
    elements3d::{DisplayMode, Mesh3D, Transform3D, Vec3D, Viewport},
    gameloop::{sleep_fps, MainLoopRoot},
};

mod debug_manager;
pub use debug_manager::DebugManager;

#[allow(dead_code)]
pub struct Root {
    canvas: ScaleFitView,
    viewport: Viewport,
    model_animation: Vec3D,
    models: Vec<Mesh3D>,
    display_mode: DisplayMode,
    shader: Box<dyn CanShade>,
    // Debug
    debug_manager: DebugManager,
}

impl Root {
    pub fn new(
        canvas: ScaleFitView,
        fov: f64,
        model_animation: Vec3D,
        models: Vec<Mesh3D>,
        display_mode: DisplayMode,
        shader: impl CanShade + 'static,
        debug_manager: DebugManager,
    ) -> Self {
        let viewport_center = canvas.intended_size() / 2;

        Self {
            canvas,
            viewport: Viewport::new(Transform3D::default(), fov, viewport_center),
            models,
            model_animation,
            display_mode,
            shader: Box::new(shader),
            debug_manager,
        }
    }
}

impl MainLoopRoot for Root {
    type InputDataType = u8;

    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        for model in &mut self.models {
            model.transform.rotation += self.model_animation;
        }
    }

    fn render_frame(&mut self) {
        // Auto-resize
        self.viewport.origin = self.canvas.intended_size() / 2;
        self.canvas.update();

        let now = Instant::now();

        self.canvas.view.blit(
            &self
                .viewport
                .render(self.models.iter().collect(), self.display_mode.clone())
                .shade_with(&mut self.shader),
            Wrapping::Ignore,
        );

        self.debug_manager.log_blitting_since(now);

        let now = Instant::now();
        let _ = self.canvas.view.display_render();
        self.debug_manager.log_rendering_since(now);
    }

    fn sleep_and_get_input_data(
        &self,
        fps: f32,
        elapsed: Duration,
    ) -> (bool, Option<Self::InputDataType>) {
        // Hijack the sleep function to print elapsed times before falling back to default sleep function
        self.debug_manager.print_benchmark(fps, elapsed);

        (sleep_fps(fps, Some(elapsed)), None)
    }
}
