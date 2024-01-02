mod obj;
mod stl;

use std::path::Path;
use gemini_engine::elements3d::Mesh3D;

pub enum ModelFileType {
    Obj,
    Stl
}

impl ModelFileType {
    pub fn from_filepath(filepath: &Path) -> Result<ModelFileType, String> {
        if let Some(file_extension) = filepath.extension() {
            match file_extension.to_str().unwrap() {
                "obj" => Ok(ModelFileType::Obj),
                "stl" => Ok(ModelFileType::Stl),
                _ => Err(String::from("Filetype not supported"))
            }
        } else {
            Err(String::from("Missing file extension"))
        }
    }
}

pub struct ModelFile<'a> {
	filepath: &'a Path,
	filetype: ModelFileType
}

impl<'a> ModelFile<'a> {
	pub fn new(filepath: &str) -> Result<ModelFile, String> {
		let filepath = Path::new(filepath);
		let filetype = ModelFileType::from_filepath(filepath)?;

		Ok(ModelFile {
			filepath,
			filetype
		})
	}

	pub fn to_mesh3ds(&self) -> Vec<Mesh3D> {
		match self.filetype {
			ModelFileType::Obj => {
				let (models, materials) = obj::get_obj_from_file(self.filepath);

				obj::obj_to_mesh3ds(models, materials)
			}

			ModelFileType::Stl => {
				vec![stl::stl_to_mesh3d(self.filepath)]
			}
		}
	}
}