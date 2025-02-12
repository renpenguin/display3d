use gemini_engine::{
    core::ColChar,
    mesh3d::{Face, Mesh3D, Vec3D},
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

#[cfg(test)]
mod tests {
    use super::to_mesh3d;
    use std::path::Path;

    #[test]
    fn load_stl() {
        let model = to_mesh3d(Path::new("resources/shapes.stl"));
        let Ok(model) = model else {
            panic!("{}", model.unwrap_err())
        };
        assert_eq!(model.faces.len(), 1546);
    }
}
