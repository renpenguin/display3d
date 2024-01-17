mod config;
mod convert_to_mesh3d;

use std::process;

pub use config::Config;
pub use convert_to_mesh3d::ModelFile;

/// Disables the cursor blink and re-enables it when SIGINT is called
pub fn disable_cursor_blink() {
    ctrlc::set_handler(move || {
        println!("\x1b[?25h");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    print!("\x1b[?25l");
}