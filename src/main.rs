use clap::Parser;
use gemini_engine::{
    elements::view::ScaleFitView,
    elements3d::{DisplayMode, Light, Vec3D},
    gameloop::MainLoopRoot,
};
use std::process;

use display3d::{shaders::MultiShader, Config, ModelFile, Root};

fn main() {
    let config = Config::parse();

    let model_file = match ModelFile::new(&config.filepath) {
        Ok(model) => model,
        Err(e) => {
            eprintln!("An error occurred while parsing the file: {e}");
            process::exit(1);
        }
    };
    let models = model_file.to_mesh3ds();

    println!(
        "Parsed model for a total of {} faces. Displaying...",
        models.iter().map(|m| m.faces.len()).sum::<usize>()
    );

    let mut root = Root::new(
        ScaleFitView::new(config.get_background_char()).with_empty_row_count(2),
        config.fov,
        config.get_transform(),
        models,
        DisplayMode::Illuminated {
            lights: vec![
                Light::new_ambient(0.6),
                Light::new_directional(0.4, Vec3D::new(-2.0, -1.0, 3.0)),
            ],
        },
        MultiShader::None,
        config.show_benchmark,
    );

    root.main_loop(config.fps);
}
