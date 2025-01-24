use std::time::{Duration, Instant};

use gemini_engine::{
    containers::{CanShade, PixelContainer},
    core::CanDraw,
    gameloop::{sleep_fps, MainLoopRoot},
    mesh3d::{Mesh3D, Transform3D, Vec3D},
    view::ScaleFitView,
    view3d::{DisplayMode, Viewport},
};

mod debug_manager;
pub use debug_manager::DebugManager;

use crate::Config;

pub struct Root {
    canvas: ScaleFitView,
    viewport: Viewport,
    model_animation: Transform3D,
    shader: Box<dyn CanShade>,
    fps: f32,
    // Debug
    debug_manager: DebugManager,
}

impl Root {
    #[must_use]
    pub fn new(config: &Config, models: Vec<Mesh3D>, display_mode: DisplayMode) -> Self {
        let canvas = ScaleFitView::new(config.get_background_char()).with_empty_row_count(0);

        let viewport_center = canvas.intended_size() / 2;

        let mut viewport = Viewport::new(
            Transform3D::look_at_lh(
                Vec3D::new(0.0, 0.0, 1.0),
                Vec3D::ZERO,
                Vec3D::new(0.0, 1.0, 0.0),
            ),
            config.fov,
            viewport_center,
        );
        viewport.objects = models;
        viewport.display_mode = display_mode;

        let model_animation = Transform3D::from_euler(
            glam::EulerRot::XYZ,
            config.animation.x,
            config.animation.y,
            config.animation.z,
        );

        Self {
            canvas,
            viewport,
            model_animation,
            shader: Box::new(config.shader),
            fps: config.fps,
            debug_manager: DebugManager::new(config.show_benchmark),
        }
    }
}

impl MainLoopRoot for Root {
    type InputDataType = ();

    fn get_fps(&self) -> f32 {
        self.fps
    }

    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        for model in &mut self.viewport.objects {
            model.transform = model.transform.mul_mat4(&self.model_animation);
        }
    }

    fn render_frame(&mut self) {
        // Auto-resize
        self.viewport.canvas_centre = self.canvas.intended_size() / 2;
        self.canvas.update();

        let now = Instant::now();

        PixelContainer::from(&self.viewport)
            .shade_with(&mut self.shader)
            .draw_to(&mut self.canvas.view);

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

        (sleep_fps(fps, Some(elapsed)), Some(()))
    }
}
