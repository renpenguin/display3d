use gemini_engine::mesh3d::Mesh3D;
use std::path::Path;

#[cfg(feature = "obj")]
mod obj;
#[cfg(feature = "stl")]
mod stl;

pub enum ModelFileType {
    #[cfg(feature = "obj")]
    Obj,
    #[cfg(feature = "stl")]
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
            #[cfg(feature = "obj")]
            "obj" => Ok(Self::Obj),
            #[cfg(feature = "stl")]
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
    pub fn to_mesh3d(&self) -> Result<Mesh3D, String> {
        match self.filetype {
            #[cfg(feature = "obj")]
            ModelFileType::Obj => obj::to_mesh3d(self.filepath),

            #[cfg(feature = "stl")]
            ModelFileType::Stl => Ok(stl::to_mesh3d(self.filepath)?),
        }
    }
}
