use clap::Parser;
use gemini_engine::{
    gameloop::MainLoopRoot,
    mesh3d::Vec3D,
    view3d::{DisplayMode, Light},
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
        Ok(file) => file,
        Err(e) => {
            eprintln!("An error occurred while parsing the file: {e}");
            process::exit(1);
        }
    };

    let models = match model_file.to_mesh3ds() {
        Ok(models) => models,
        Err(e) => {
            eprintln!("An error occured while converting the parsed file: {e}");
            process::exit(1);
        }
    };

    println!(
        "Parsed model for a total of {} faces. Displaying...",
        models.iter().map(|m| m.faces.len()).sum::<usize>()
    );

    // Apply global transform
    let models = models
        .into_iter()
        .map(|m| {
            let mut m = m;
            m.transform *= -config.get_transform();
            m
        })
        .collect();

    let mut root = Root::new(
        &config,
        models,
        DisplayMode::Illuminated {
            lights: vec![
                Light::new_ambient(0.6),
                Light::new_directional(0.4, Vec3D::new(2.0, 1.0, 3.0)),
            ],
        },
    );

    if config.render_once {
        root.render_frame();
        return;
    }

    init::disable_cursor_blink();

    root.main_loop();
}
