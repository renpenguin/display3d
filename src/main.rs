use std::{env, process};

use display3d::{Config, ModelFile, Root};
use gemini_engine::elements::view::View;
use gemini_engine::elements3d::{DisplayMode, Light, Vec3D};
use gemini_engine::gameloop::MainLoopRoot;

fn main() {
    let config = Config::from_args(env::args()).unwrap_or_else(|e| {
        eprintln!("An error occurred while parsing arguments: {e}");
        process::exit(1);
    });

    let model_file = match ModelFile::new(&config.filepath) {
        Ok(model) => model,
        Err(e) => {
            eprintln!("An error occurred while parsing the file: {e}");
            process::exit(1);
        }
    };

    let mut root = Root::new(
        View::new(0, 0, config.background_colchar).with_block_until_resized(true),
        config.fov,
        config.viewport_transform,
        model_file.to_mesh3ds(),
        DisplayMode::Illuminated {
            lights: vec![
                Light::new_ambient(0.6),
                Light::new_directional(0.4, Vec3D::new(-2.0, -1.0, 3.0)),
            ],
        },
        config.show_benchmark,
    );

    root.main_loop(config.fps);
}
