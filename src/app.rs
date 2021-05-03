use std::fs;
use std::path::{Path, PathBuf};

use directories_next::ProjectDirs;

use crate::error::KeyManagerError;

pub const APP_QUALIFIER: &'static str = "com";
pub const APP_ORGANIZATION: &'static str = "Gravitational Marketing";
pub const APP_NAME: &'static str = "SSH Keyserver Manager";

pub struct Application {
    pub project_dir: ProjectDirs,
}

impl Application {
    pub fn create() -> Result<Self, KeyManagerError> {
        Ok(Self {
            project_dir: ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
                .ok_or(KeyManagerError::InvalidProjectDirectory)?,
        })
    }

    pub fn get_group(&self, group: &str) -> Group {
        Group {
            name: group.to_owned(),
            path: PathBuf::from(self.project_dir.data_dir())
                .join(group),
        }
    }

    pub fn get_groups(&self) -> Result<Vec<Group>, KeyManagerError> {
        let paths = list_dir(self.project_dir.data_dir());

        match paths {
            Ok(paths) => Ok(paths.iter()
                .map(|path| Group {
                    path: path.to_owned(),
                    name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                })
                .collect::<Vec<_>>()),
            Err(e) => Err(e),
        }
    }
}

fn list_dir<T: AsRef<Path>>(base: T) -> Result<Vec<PathBuf>, KeyManagerError> {
    let mut res = fs::read_dir(base)
        .map_err(|e| KeyManagerError::IoError(e))?
        .map(|res| {
            res
                .map(|entry| entry.path())
                .map_err(|error| KeyManagerError::UnknownError(Box::new(error)))
        })
        .collect::<Result<Vec<_>, KeyManagerError>>()?;

    res.sort();

    Ok(res)
}

pub struct Group {
    pub path: PathBuf,
    pub name: String,
}

impl Group {
    pub fn put(&self, key_name: &str, public_key: &str) -> Result<(), KeyManagerError> {
        if !self.path.exists() {
            fs::create_dir_all(&self.path).map_err(|e| {
                KeyManagerError::IoError(e)
            })?;
        }

        let key_path = self.get_path(key_name);

        fs::write(&key_path, public_key).map_err(|e| {
            KeyManagerError::IoError(e)
        })
    }

    pub fn items(&self) -> Result<Vec<PathBuf>, KeyManagerError> {
        list_dir(&self.path)
    }

    pub fn get_path(&self, key_name: &str) -> PathBuf {
        PathBuf::from(&self.path)
            .join(key_name)
    }
}
