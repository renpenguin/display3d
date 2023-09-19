use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::elements3d::{DisplayMode, Mesh3D, Transform3D, Vec3D, Viewport};
use gemini_engine::{fps_gameloop, gameloop};
use obj_view::obj_to_mesh3ds;

const OBJ_FILEPATH: &str = "obj-view/resources/ren.obj";
// const MTL_FILEPATH: &str = "obj-view/model.mtl";
const WIDTH: usize = 370;
const HEIGHT: usize = 90;
const FPS: f32 = 60.0;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(WIDTH, HEIGHT, ColChar::BACKGROUND);

    let mut viewport = Viewport::new(
        Transform3D::new_tr(Vec3D::new(0.0, -0.7, 2.2), Vec3D::new(-0.3, 0.0, 0.0)),
        FOV,
        view.center(),
    );
    viewport.character_width_multiplier = 2.2;

    let load_options = tobj::LoadOptions::default();
    let (models, materials) =
        tobj::load_obj(OBJ_FILEPATH, &load_options).expect("Failed to OBJ load file");
    let materials = materials.unwrap_or(vec![]); // TODO: fallback to MTL_FILEPATH

    let mesh3d_models: Vec<Mesh3D> = obj_to_mesh3ds(models, materials);

    let mut elapsed_blitting;
    let mut elapsed_rendering;
    fps_gameloop!(
        {
            // Logic
            view.clear();
            viewport.transform.rotation.y += 0.05;
            elapsed_blitting = None;
            elapsed_rendering = None;
        },
        {
            // Rendering
            let now_blitting = gameloop::Instant::now();
            view.blit(
                &viewport.render(mesh3d_models.iter().collect(), DisplayMode::Solid),
                Wrapping::Ignore,
            );
            elapsed_blitting = Some(now_blitting.elapsed());

            let now_rendering = gameloop::Instant::now();
            view.display_render().unwrap();
            elapsed_rendering = Some(now_rendering.elapsed());
        },
        FPS,
        |elapsed: gameloop::Duration, frame_skip| {
            // Timing info
            println!(
            "Elapsed - Blitting: {:.2?}µs, Rendering: {:.2?}µs, Total: {:.2?}µs | Frame skip: {}",
            elapsed_blitting
                .unwrap_or(gameloop::Duration::default())
                .as_micros(),
            elapsed_rendering
                .unwrap_or(gameloop::Duration::default())
                .as_micros(),
            elapsed.as_micros(),
            frame_skip
        );
        }
    );
}
