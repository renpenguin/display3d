use gemini_engine::elements::{view::ColChar, View};
use gemini_engine::elements3d::{DisplayMode, Face, Mesh3D, Vec3D, Viewport};
use gemini_engine::gameloop;
use tobj;

const OBJ_FILEPATH: &str = "obj-view/model.obj";
// const MTL_FILEPATH: &str = "obj-view/model.mtl";
const WIDTH: usize = 370;
const HEIGHT: usize = 100;
const FPS: u32 = 60;
const FOV: f64 = 95.0;

fn main() {
    let mut view = View::new(WIDTH, HEIGHT, ColChar::BACKGROUND);
    let mut frame_skip = false;

    let mut viewport = Viewport::new(
        Vec3D::new(0.0, -0.7, 2.0),
        Vec3D::new(-0.3, 0.0, 0.0),
        FOV,
        view.center(),
    );
    viewport.character_width_multiplier = 2.2;

    let (models, materials) = tobj::load_obj(OBJ_FILEPATH, &tobj::LoadOptions::default())
        .expect("Failed to OBJ load file");
    let materials = materials.unwrap_or(vec![]); // TODO: fallback to MTL_FILEPATH

    let mesh3d_models: Vec<Mesh3D> = models
        .iter()
        .map(|model| {
            let mesh = &model.mesh;

            let mut next_face = 0;
            let faces: Vec<Face> = (0..mesh.face_arities.len())
                .map(|f| {
                    let end = next_face + mesh.face_arities[f] as usize;
                    let face_indices = mesh.indices[next_face..end]
                        .iter()
                        .map(|i| *i as usize)
                        .rev()
                        .collect();

                    let material = match mesh.material_id {
                        Some(material_id) => materials[material_id].diffuse.unwrap(),
                        None => [1.0, 0.0, 1.0],
                    };

                    next_face = end;
                    Face::new(
                        face_indices,
                        ColChar::SOLID.with_rgb(
                            (material[0] * 255.0) as u8,
                            (material[1] * 255.0) as u8,
                            (material[2] * 255.0) as u8,
                        ),
                    )
                })
                .collect();

            Mesh3D::new(
                Vec3D::ZERO,
                Vec3D::ZERO,
                mesh.positions
                    .chunks(3)
                    .map(|v| Vec3D::new(v[0].into(), v[1].into(), v[2].into()))
                    .collect(),
                faces,
            )
        })
        .collect();

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
