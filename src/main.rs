use clap::Parser;
use gemini_engine::{
    elements::view::ScaleFitView,
    elements3d::{DisplayMode, Light, Vec3D},
    gameloop::MainLoopRoot,
};
use std::process;

mod display_model;
mod init;
mod shaders;

pub use crate::{
    display_model::{DebugManager, Root},
    init::{Config, ModelFile},
    shaders::MultiShader,
};

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
        config.shader,
        DebugManager::new(config.show_benchmark, config.stop_after),
    );

    init::disable_cursor_blink();

    root.main_loop(config.fps);
}
