use gemini_engine::elements::{view::ColChar, View};
use gemini_engine::elements3d::{DisplayMode, Mesh3D, Vec3D, Viewport};
use gemini_engine::gameloop;
use obj_view::obj_to_mesh3ds;
use tobj;

const OBJ_FILEPATH: &str = "obj-view/resources/ren.obj";
// const MTL_FILEPATH: &str = "obj-view/model.mtl";
const WIDTH: usize = 370;
const HEIGHT: usize = 90;
const FPS: u32 = 60;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(WIDTH, HEIGHT, ColChar::BACKGROUND);
    let mut frame_skip = false;

    let mut viewport = Viewport::new(
        Vec3D::new(0.0, -0.7, 2.2),
        Vec3D::new(-0.3, 0.0, 0.0),
        FOV,
        view.center(),
    );
    viewport.character_width_multiplier = 2.2;

    let (models, materials) = tobj::load_obj(OBJ_FILEPATH, &tobj::LoadOptions::default())
        .expect("Failed to OBJ load file");
    let materials = materials.unwrap_or(vec![]); // TODO: fallback to MTL_FILEPATH

    let mesh3d_models: Vec<Mesh3D> = obj_to_mesh3ds(models, materials);

    loop {
        let now_blitting = gameloop::Instant::now();
        let now = gameloop::Instant::now();
        let mut elapsed_blitting = None;
        let mut now_rendering = None;
        view.clear();

        viewport.rotation.y += 0.05;

        match frame_skip {
            true => frame_skip = false,
            false => {
                viewport.blit_to(
                    &mut view,
                    mesh3d_models.iter().collect(),
                    DisplayMode::Solid,
                );

                elapsed_blitting = Some(now_blitting.elapsed());
                now_rendering = Some(gameloop::Instant::now());
                view.display_render().unwrap();
            }
        }

        let elapsed = now.elapsed();
        let elapsed_rendering = now_rendering.unwrap_or(gameloop::Instant::now()).elapsed();
        println!(
            "Elapsed - Blitting: {:.2?}µs, Rendering: {:.2?}µs, Total: {:.2?}µs | Frame skip: {}",
            elapsed_blitting
                .unwrap_or(gameloop::Duration::default())
                .as_micros(),
            elapsed_rendering.as_micros(),
            elapsed.as_micros(),
            frame_skip
        );

        frame_skip = gameloop::sleep_fps(FPS, Some(elapsed));
    }
}
