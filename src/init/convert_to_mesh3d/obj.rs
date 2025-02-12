use gemini_engine::{
    core::ColChar,
    mesh3d::{Face, Mesh3D, Vec3D},
};
use std::path::Path;
use tobj::{Material, Model as ObjModel};

const NO_MATERIAL_COLOUR: [f32; 3] = [1.0, 0.0, 1.0];

fn get_material_as_col_char(materials: &[Material], material_id: Option<usize>) -> ColChar {
    let colour_rgb = material_id
        .and_then(|id| materials[id].diffuse)
        .unwrap_or(NO_MATERIAL_COLOUR);

    ColChar::SOLID.with_rgb(
        (colour_rgb[0] * 255.0) as u8,
        (colour_rgb[1] * 255.0) as u8,
        (colour_rgb[2] * 255.0) as u8,
    )
}

fn extend_mesh3d_with_objmodel(mesh: &mut Mesh3D, obj_model: &ObjModel, materials: &[Material]) {
    let obj_model = &obj_model.mesh;

    let index_offset = mesh.vertices.len();

    // let all_texcoords: Vec<Vector2<f64>> = mesh.texcoords.chunks(2).map(|k| Vector2::new(k[0] as f64, k[1] as f64)).collect();
    // let indexed_texcoords: Vec<Vector2<f64>> = mesh.texcoord_indices.iter().map(|i| all_texcoords[*i as usize]).collect();

    mesh.vertices.extend(
        obj_model
            .positions
            .chunks(3)
            .map(|v| Vec3D::new(v[0].into(), v[1].into(), v[2].into())),
    );

    if obj_model.face_arities.is_empty() {
        obj_model.indices.chunks(3).for_each(|v| {
            mesh.faces.push(Face::new(
                v.iter().map(|i| *i as usize + index_offset).collect(),
                get_material_as_col_char(materials, obj_model.material_id),
            ));
        });
    } else {
        let mut next_face = 0;
        for i in 0..obj_model.face_arities.len() {
            let end = next_face + obj_model.face_arities[i] as usize;
            let v = &obj_model.indices[next_face..end];
            next_face = end;

            mesh.faces.push(Face::new(
                v.iter().map(|i| *i as usize + index_offset).collect(),
                get_material_as_col_char(materials, obj_model.material_id),
            ));
        }
    };
}

pub fn to_mesh3d(filepath: &Path) -> Result<Mesh3D, String> {
    let (models, materials) = get_obj_from_file(filepath)?;

    let mut final_model = Mesh3D::new(Vec::new(), Vec::new());

    for model in models {
        extend_mesh3d_with_objmodel(&mut final_model, &model, &materials);
    }

    Ok(final_model)
}

fn get_obj_from_file(obj_filepath: &Path) -> Result<(Vec<ObjModel>, Vec<Material>), String> {
    let load_options = tobj::LoadOptions::default();

    let (models, materials) =
        tobj::load_obj(obj_filepath, &load_options).map_err(|e| e.to_string())?;

    let materials = materials.unwrap_or(vec![]);

    Ok((models, materials))
}

#[cfg(test)]
mod tests {
    use super::to_mesh3d;
    use gemini_engine::core::Modifier;
    use std::path::Path;

    #[test]
    fn load_obj() {
        let model = to_mesh3d(Path::new("resources/blahaj.obj"));
        let Ok(model) = model else {
            panic!("{}", model.unwrap_err())
        };
        assert_eq!(model.faces.len(), 268);
        assert_eq!(
            model.faces[0].fill_char.modifier,
            Modifier::from_rgb(97, 158, 176)
        ); // Assert successful .mtl file parse
    }
}
