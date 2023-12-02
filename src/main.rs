use std::env;

use gemini_engine::elements::view::{ColChar, View};
use gemini_engine::elements3d::{DisplayMode, Light, Mesh3D, Transform3D, Vec3D};
use gemini_engine::gameloop::MainLoopRoot;
use obj_view::Root;

// const OBJ_FILEPATH: &str = "obj-view/resources/ren.obj";
// const MTL_FILEPATH: &str = "obj-view/model.mtl";
const WIDTH: usize = 370;
const HEIGHT: usize = 90;
const FPS: f32 = 60.0;
const FOV: f64 = 95.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let obj_filepath = &args[1];

    let (models, materials) = obj_view::get_obj_from_file(obj_filepath);
    let mesh3d_models: Vec<Mesh3D> = obj_view::obj_to_mesh3ds(models, materials);

    let mut root = Root::new(
        View::new(
            WIDTH,
            HEIGHT,
            ColChar::SOLID.with_rgb(50, 40, 40).with_char('$'),
        )
        .with_block_until_resized(true),
        FOV,
        Transform3D::new_tr(Vec3D::new(0.0, -0.7, 2.2), Vec3D::new(-0.3, 0.0, 0.0)),
        mesh3d_models,
        8,
        DisplayMode::Illuminated {
            lights: vec![
                Light::new_ambient(0.3),
                Light::new_directional(0.7, Vec3D::new(-2.0, -1.0, 3.0)),
            ],
        },
    );

    root.main_loop(FPS);
}
