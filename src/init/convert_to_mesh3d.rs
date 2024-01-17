mod obj;
mod stl;

use gemini_engine::elements3d::Mesh3D;
use std::path::Path;

fn error_to_string<E: ToString>(error: E) -> String {
    error.to_string()
}

pub enum ModelFileType {
    Obj,
    Stl,
}

impl ModelFileType {
    pub fn from_filepath(filepath: &Path) -> Result<ModelFileType, String> {
        if let Some(file_extension) = filepath.extension() {
            if let Some(extension) = file_extension.to_str() {
                match extension {
                    "obj" => Ok(ModelFileType::Obj),
                    "stl" => Ok(ModelFileType::Stl),
                    _ => Err(String::from("Filetype not supported")),
                }
            } else {
                Err(String::from("File extension is not a valid OsStr"))
            }
        } else {
            Err(String::from("Missing file extension"))
        }
    }
}

pub struct ModelFile<'a> {
    filepath: &'a Path,
    filetype: ModelFileType,
}

impl<'a> ModelFile<'a> {
    pub fn new(filepath: &str) -> Result<ModelFile, String> {
        let filepath = Path::new(filepath);
        let filetype = ModelFileType::from_filepath(filepath)?;

        Ok(ModelFile { filepath, filetype })
    }

    pub fn to_mesh3ds(&self) -> Result<Vec<Mesh3D>, String> {
        match self.filetype {
            ModelFileType::Obj => obj::obj_to_mesh3ds(self.filepath),

            ModelFileType::Stl => Ok(vec![stl::stl_to_mesh3d(self.filepath)?]),
        }
    }
}
