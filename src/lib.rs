use gemini_engine::{
    elements::view::ColChar,
    elements3d::{IndexFace as Face, Mesh3D, Transform3D, Vec3D},
};
use tobj::{Material, Model};

pub fn get_mateial_as_col_char(materials: &Vec<Material>, material_id: Option<usize>) -> ColChar {
    let colour_rgb = match material_id {
        Some(material_id) => materials[material_id].diffuse.unwrap(),
        None => [1.0, 0.0, 1.0],
    };

    ColChar::SOLID.with_rgb(
        (colour_rgb[0] * 255.0) as u8,
        (colour_rgb[1] * 255.0) as u8,
        (colour_rgb[2] * 255.0) as u8,
    )
}

pub fn model_to_mesh3d(model: &Model, materials: &Vec<Material>) -> Mesh3D {
    {
        let mesh = &model.mesh;

        let vertices = mesh
            .positions
            .chunks(3)
            .map(|v| Vec3D::new(v[0].into(), v[1].into(), v[2].into()))
            .collect();

        let mut next_face = 0;
        let faces: Vec<Face> = match mesh.face_arities.len() {
            // If face_arities is empty (triangulate is on or mesh consists of triangles only)
            0 => mesh
                .indices
                .chunks(3)
                .map(|v| {
                    Face::new(
                        v.iter().map(|i| *i as usize).collect(),
                        get_mateial_as_col_char(&materials, mesh.material_id),
                    )
                })
                .collect(),
            // Otherwise
            _ => (0..mesh.face_arities.len())
                .map(|f| {
                    let end = next_face + mesh.face_arities[f] as usize;
                    let face_indices = mesh.indices[next_face..end]
                        .iter()
                        .map(|i| *i as usize)
                        .rev()
                        .collect();

                    let material = get_mateial_as_col_char(&materials, mesh.material_id);

                    next_face = end;
                    Face::new(face_indices, material)
                })
                .collect(),
        };

        Mesh3D::with_faces(Transform3D::DEFAULT, vertices, faces)
    }
}

pub fn obj_to_mesh3ds(models: Vec<Model>, materials: Vec<Material>) -> Vec<Mesh3D> {
    models
        .iter()
        .map(|model| model_to_mesh3d(model, &materials))
        .collect()
}
