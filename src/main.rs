use std::env;

use gemini_engine::elements::view::{ColChar, View};
use gemini_engine::elements3d::{Mesh3D, Transform3D, Vec3D};
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
        View::new(WIDTH, HEIGHT, ColChar::SOLID.with_rgb(40, 40, 40)),
        FOV,
        Transform3D::new_tr(Vec3D::new(0.0, -0.7, 4.2), Vec3D::new(-0.3, 0.0, 0.0)),
        mesh3d_models,
        8,
    );

    root.main_loop(FPS);
}
