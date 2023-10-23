mod obj_to_mesh3d;
use std::time::{Duration, Instant};

use gemini_engine::{
    elements::{
        view::{ColChar, Wrapping},
        View,
    },
    elements3d::{DisplayMode, Grid3D, Mesh3D, Transform3D, ViewElement3D, Viewport, view3d::Light, Vec3D},
    gameloop::{sleep_fps, MainLoopRoot},
};
pub use obj_to_mesh3d::{get_obj_from_file, obj_to_mesh3ds};

#[allow(dead_code)]
pub struct Root {
    view: View,
    viewport: Viewport,
    lights: Vec<Light>,
    models: Vec<Mesh3D>,
    grid: Grid3D,
    // Timing stats
    elapsed_blitting: Duration,
    elapsed_rendering: Duration,
}

impl Root {
    pub fn new(
        canvas: View,
        fov: f64,
        initial_viewport_transform: Transform3D,
        models: Vec<Mesh3D>,
        cell_count: usize,
    ) -> Root {
        let viewport_center = canvas.center();
        Root {
            view: canvas,
            viewport: Viewport::new(initial_viewport_transform, fov, viewport_center),
            lights: vec![
                Light::new_ambient(0.3),
                Light::new_directional(0.7, Vec3D::new(-2.0, -1.0, 3.0)),
            ],
            models,
            grid: Grid3D::new(1.0, cell_count, ColChar::BACKGROUND),
            elapsed_blitting: Duration::ZERO,
            elapsed_rendering: Duration::ZERO,
        }
    }
}

impl MainLoopRoot for Root {
    type InputDataType = u8;
    fn frame(&mut self, _input_data: Option<Self::InputDataType>) {
        self.viewport.transform.rotation.y += 0.05;
        for model in self.models.iter_mut() {
            model.transform.translation.y = -0.2
        }
    }

    fn render_frame(&mut self) {
        self.view.clear();
        let now = Instant::now();

        // Render grid first so it never appears over any objects
        // self.view.blit(
        //     &self.viewport.render(
        //         vec![&self.grid],
        //         DisplayMode::Wireframe {
        //             backface_culling: false,
        //         },
        //     ),
        //     Wrapping::Ignore,
        // );
        let objects: Vec<&dyn ViewElement3D> = self.models.iter().map(|m| m as _).collect();
        self.view.blit(
            &self.viewport.render(objects, DisplayMode::Illuminated { lights: self.lights.clone() }),
            Wrapping::Ignore,
        );

        self.elapsed_blitting = now.elapsed();

        let now = Instant::now();
        self.view.display_render().unwrap();
        self.elapsed_rendering = now.elapsed();
    }

    fn sleep_and_get_input_data(
        &self,
        fps: f32,
        elapsed: Duration,
    ) -> (bool, Option<Self::InputDataType>) {
        // Hijack the sleep function to print elapsed times before falling back to default sleep function
        println!(
            "Elapsed - Blitting: {:.2?}µs, Rendering: {:.2?}µs, Total: {:.2?}µs",
            self.elapsed_blitting.as_micros(),
            self.elapsed_rendering.as_micros(),
            elapsed.as_micros()
        );

        (sleep_fps(fps, Some(elapsed)), None)
    }
}
