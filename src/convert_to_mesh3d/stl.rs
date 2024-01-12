use gemini_engine::{
    elements::view::ColChar,
    elements3d::{Face, Mesh3D, Transform3D, Vec3D},
};
use std::{fs::OpenOptions, path::Path};

pub fn stl_to_mesh3d(filepath: &Path) -> Mesh3D {
    let mut file = OpenOptions::new().read(true).open(filepath).unwrap();
    let mut stl = stl_io::create_stl_reader(&mut file).unwrap();
    let mut indexed_mesh = stl.as_indexed_triangles().unwrap();
    indexed_mesh
        .faces
        .iter_mut()
        .for_each(|face| face.vertices.reverse()); // Flip normals

    Mesh3D::new(
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
    )
}
