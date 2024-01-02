use gemini_engine::{
    elements::view::ColChar,
    elements3d::{Transform3D, Vec3D},
};
use std::{path::Path, str::FromStr};

fn get_result_or_return_error<T, E>(value: Result<T, E>, error: &str) -> Result<T, String> {
    match value {
        Ok(value) => Ok(value),
        Err(_) => Err(String::from(error)),
    }
}

fn try_get<'a>(iter: &'a [String], index: usize, error: &str) -> Result<&'a String, String> {
    iter.get(index).ok_or(String::from(error))
}

fn parse_next_argument<F: FromStr>(iter: &[String], index: &mut usize) -> Result<F, String> {
    *index += 1;
    get_result_or_return_error(
        try_get(iter, *index, "Not enough arguments")?.parse(),
        &format!("Couldnt parse argument for {}", iter[*index - 1]),
    )
}

pub struct Config {
    pub filepath: String,

    // 3D Space
    pub viewport_transform: Transform3D,
    pub fov: f64,
    pub fps: f32,

    // Display
    pub background_colchar: ColChar,

    // Debug
    pub show_benchmark: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            String::new(),
            Transform3D::new_tr(Vec3D::new(0.0, 0.0, 3.2), Vec3D::new(-0.3, 0.0, 0.0)),
            95.0,
            60.0,
            ColChar::EMPTY,
            false,
        )
    }
}

impl Config {
    pub fn new(
        filepath: String,
        viewport_transform: Transform3D,
        fov: f64,
        fps: f32,
        background_colchar: ColChar,
        show_benchmark: bool,
    ) -> Self {
        Self {
            filepath,
            viewport_transform,
            fov,
            fps,
            background_colchar,
            show_benchmark,
        }
    }

    pub fn from_args(args: impl Iterator<Item = String>) -> Result<Config, String> {
        let args: Vec<String> = args.collect();

        if args.len() < 2 {
            return Err(String::from(
                "Not enough arguments, please provide a filepath",
            ));
        }

        let mut config = Config::default();

        match Path::new(&args[1]).try_exists() {
            Ok(exists) => match exists {
                true => config.filepath = args[1].to_string(),
                false => return Err(String::from("Filepath does not exist")),
            },
            Err(err) => return Err(err.to_string()),
        }

        if args.len() > 2 {
            // Apply other arguments here
            let mut i = 1;
            while i < args.len() - 1 {
                i += 1;
                match args[i].as_str() {
					// TODO: viewport_position
					// TODO: viewport_rotation
                    "--fov" => {
                        config.fov = parse_next_argument(&args, &mut i)?;
                    }
                    "--fps" => {
                        config.fps = parse_next_argument(&args, &mut i)?;
                    }
                    "--background-char" => {
                        config.background_colchar.text_char = parse_next_argument(&args, &mut i)?;
                    }
					// TODO: background_colour
                    "--show-benchmark" => config.show_benchmark = true,
                    _ => return Err(String::from("Invalid argument")),
                }
            }
        }

        Ok(config)
    }
}
