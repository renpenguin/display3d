<div align="center">
  <h1><b>display3d</b></h1>
  <img src="https://img.shields.io/github/last-commit/redpenguinyt/display3d?color=%23aa0000&style=for-the-badge">
  <img src="https://img.shields.io/github/repo-size/redpenguinyt/display3d?color=%2300aa00&style=for-the-badge">
  <img src="https://img.shields.io/github/stars/redpenguinyt/display3d?color=%2300e7&style=for-the-badge">
</div>

[video example](https://github.com/redpenguinyt/display3d/assets/79577742/7c37fdaf-bb30-4f3e-9d3b-215b1d8e5cf9)

A command line interface to display and animate 3D objects using ANSI escape codes, written using [gemini-engine](https://crates.io/crates/gemini-engine) in the Rust programming language

For a basic guide on how to use the tool, run `display3d --help`

# Supported formats
- .obj (combined with .mtl for colours)
- .stl

Please note that this form of rendering requires the terminal to support ANSI escape codes, support for which tends to be unreliable on Windows from installation to installation. If you are having issues getting an image on Linux or MacOS, please submit an issue request

# Installing

Make sure you have [rust and cargo](https://www.rust-lang.org/tools/install) installed before proceeding with either of these methods

## Install with cargo (recommended)
Run `cargo install display3d`. This will download, compile and install the latest stable release of display3d. You can then simply run `display3d` to access it

## Compile from source
Clone this repository with `git clone https://github.com/redpenguinyt/display3d.git` or download and extract this repository.

Build the project with `cargo build --release`. The binary will be saved to `<project-repo>/target/release/display3d`

# Example
To run the example spinning shark, run `display3d resources/blahaj.obj -t 0,0,5.5`
