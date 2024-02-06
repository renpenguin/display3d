use gemini_engine::{
    elements::view::ColChar,
    elements3d::{Face, Mesh3D, Transform3D, Vec3D},
};
use std::{fs::OpenOptions, path::Path};

pub fn to_mesh3d(filepath: &Path) -> Result<Mesh3D, String> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(filepath)
        .map_err(|e| e.to_string())?;
    let mut stl = stl_io::create_stl_reader(&mut file).map_err(|e| e.to_string())?;
    let mut indexed_mesh = stl.as_indexed_triangles().map_err(|e| e.to_string())?;

    indexed_mesh
        .faces
        .iter_mut()
        .for_each(|face| face.vertices.reverse()); // Flip normals

    Ok(Mesh3D::new(
        Transform3D::default(),
        indexed_mesh
            .vertices
            .iter()
            .map(|v| Vec3D::from((v[0], v[1], v[2])))
            .collect(),
        indexed_mesh
            .faces
            .iter()
            .map(|f| Face::new(f.vertices.to_vec(), ColChar::SOLID))
            .collect(),
    ))
}
