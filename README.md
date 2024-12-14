<div align="center">
  <h1><b>display3d</b></h1>
  <img alt="Crates.io Version" src="https://img.shields.io/crates/v/display3d?style=for-the-badge">
  <img src="https://img.shields.io/github/last-commit/renpenguin/display3d?style=for-the-badge">
  <img src="https://img.shields.io/github/repo-size/renpenguin/display3d?style=for-the-badge">
  <img src="https://img.shields.io/github/stars/renpenguin/display3d?color=e4b400&style=for-the-badge">
</div>

[demo video of display3d](https://github.com/renpenguin/display3d/assets/79577742/6131167a-7b83-4c8e-96ec-c9715f3b4d23)

display3d is a command line interface for rendering and animating 3D objects using ANSI escape codes, written using [gemini-engine](https://crates.io/crates/gemini-engine) in the Rust programming language.

For a basic guide on how to use the tool, run `display3d --help`

# Supported formats

- .obj (combined with .mtl for colours). If you're exporting from Blender, set the forward axis to +Z and the up axis to +Y
- .stl

Please note that this form of rendering requires the terminal to support ANSI escape codes, support for which tends to be unreliable on Windows from installation to installation. If you are having issues getting an image on Linux or MacOS, please submit an issue request

# Installing

## Arch Linux

[display3d](https://aur.archlinux.org/packages/display3d/) is available as a package in the [AUR](https://aur.archlinux.org/). You can install it using your preferred AUR helper, e.g. `paru -S display3d`.

## Other

Make sure you have [rust and cargo](https://www.rust-lang.org/tools/install) installed before proceeding with either of these methods

### Cargo

Run `cargo install display3d`. This will download, compile and install the latest stable release of display3d. You can then simply run `display3d` to access it

### Compile from source

Clone this repository with `git clone https://github.com/renpenguin/display3d.git` or download and extract this repository.

Build the project with `cargo build --release`. The binary will be saved to `<project-repo>/target/release/display3d`

# Example

To run the example spinning shark, run `display3d blahaj.obj -t 0,0,5.5`. You can get the `blahaj.obj` and `blahaj.mtl` (for colours, should be stored together with `blahaj.obj`) files in [the resource folder](https://github.com/renpenguin/display3d/tree/master/resources) of this repository.
