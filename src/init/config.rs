use clap::Parser;
use gemini_engine::{
    elements::view::{ColChar, Modifier},
    elements3d::{Transform3D, Vec3D},
};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    allow_hyphen_values = true
)]
pub struct Config {
    /// The filepath of the 3D model
    #[arg()]
    pub filepath: String,

    /// The translation of the viewport/camera
    #[arg(short, long, default_value_t = Vec3D::new(0.0,-0.5,5.0))]
    pub translation: Vec3D,

    /// The rotation of the viewport/camera
    #[arg(short, long, default_value_t = Vec3D::new(-0.2,0.0,0.0))]
    pub rotation: Vec3D,

    /// The FOV of the viewport
    #[arg(long, default_value_t = 95.0)]
    pub fov: f64,
    /// The FPS at which the animation should run
    #[arg(long, default_value_t = 60.0)]
    pub fps: f32,

    /// Character used by the background
    #[arg(long, default_value_t = ' ')]
    pub background_char: char,
    /// ANSI Code to modify background, see <https://wikipedia.org/wiki/ANSI_escape_code#Colors>
    #[arg(short, long, default_value_t = 0)]
    pub background_modifier_code: u8,

    // Debug
    /// Whether to show render times below the rendered image
    #[arg(long, default_value_t = false)]
    pub show_benchmark: bool,

    /// How many frames to stop rendering after. Set to 0 to disable (this is the default)
    #[arg(long, default_value_t = 0)]
    pub stop_after: usize,
}

impl Config {
    pub fn get_background_char(&self) -> ColChar {
        ColChar::new(
            self.background_char,
            Modifier::Coded(self.background_modifier_code),
        )
    }

    pub fn get_transform(&self) -> Transform3D {
        Transform3D::new_tr(self.translation, self.rotation)
    }
}
