mod obj;
mod stl;

use gemini_engine::elements3d::Mesh3D;
use std::path::Path;

pub enum ModelFileType {
    Obj,
    Stl,
}

impl ModelFileType {
    pub fn from_filepath(filepath: &Path) -> Result<Self, String> {
        let Some(file_extension) = filepath.extension() else {
            return Err(String::from("Missing file extension"));
        };

        let Some(extension) = file_extension.to_str() else {
            return Err(String::from("File extension is not a valid OsStr"));
        };

        match extension {
            "obj" => Ok(Self::Obj),
            "stl" => Ok(Self::Stl),
            _ => Err(String::from("Filetype not supported")),
        }
    }
}

pub struct ModelFile<'a> {
    filepath: &'a Path,
    filetype: ModelFileType,
}

impl<'a> ModelFile<'a> {
    /// ## Errors
    /// Will return an error if unable to determine a valid file extension based on the filepath
    pub fn new(filepath: &str) -> Result<ModelFile, String> {
        let filepath = Path::new(filepath);
        let filetype = ModelFileType::from_filepath(filepath)?;

        Ok(ModelFile { filepath, filetype })
    }

    /// ## Errors
    /// Returns errors either from converting the obj or stl. These are presented as a printable string for reporting the issue directly to the user
    pub fn to_mesh3ds(&self) -> Result<Vec<Mesh3D>, String> {
        match self.filetype {
            ModelFileType::Obj => obj::to_mesh3ds(self.filepath),

            ModelFileType::Stl => Ok(vec![stl::to_mesh3d(self.filepath)?]),
        }
    }
}
