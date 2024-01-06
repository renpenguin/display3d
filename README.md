A command line interface to display and animate 3D objects using ANSI escape codes, written using [gemini-engine](https://crates.io/crates/gemini-engine) in the Rust programming language

For a basic guide on how to use the tool, run `display3d --help`

## Supported formats
- .obj (combined with .mtl for colours)
- .stl

Please note that this form of rendering requires the terminal to support ANSI escape codes, support for which tends to be unreliable on Windows from installation to installation. If you are having issues getting an image on Linux or MacOS, please submit an issue request

# Installing

## Compile from source
Clone this repository with `git clone https://github.com/redpenguinyt/display3d.git` or download and extract this repository.

Make sure you have [rust and cargo](https://www.rust-lang.org/tools/install) installed, and build the project with `cargo build --release`. The binary will be saved to `<project-repo>/target/release/display3d`