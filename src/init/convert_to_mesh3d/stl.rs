use gemini_engine::{
    core::ColChar,
    mesh3d::{Mesh3D, Vec3D, Face},
};
use std::{fs::OpenOptions, path::Path};

pub fn to_mesh3d(filepath: &Path) -> Result<Mesh3D, String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(filepath)
        .map_err(|e| e.to_string())?;

    let mut stl = stl_io::create_stl_reader(&mut file).map_err(|e| e.to_string())?;

    let indexed_mesh = stl.as_indexed_triangles().map_err(|e| e.to_string())?;

    Ok(Mesh3D::new(
        indexed_mesh
            .vertices
            .into_iter()
            .map(|v| Vec3D::new(v[0].into(), v[1].into(), v[2].into()))
            .collect(),
        indexed_mesh
            .faces
            .into_iter()
            .map(|f| Face::new(f.vertices.to_vec(), ColChar::SOLID))
            .collect(),
    ))
}
