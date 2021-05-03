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
        let mut paths = list_dir(self.project_dir.data_dir())?;

        Ok(
            paths.drain(..)
                .filter_map(|path| {
                    if path.is_dir() {
                        Some(Group {
                            name: path.file_name().unwrap().to_str().unwrap().to_owned(),
                            path,
                        })
                    } else {
                        None
                    }
                })
                .collect()
        )
    }

    pub fn mark_dirty(&self) -> Result<(), KeyManagerError> {
        match fs::OpenOptions::new().create(true).write(true).open(self.get_dirty_path()) {
            Ok(_) => Ok(()),
            Err(e) => Err(KeyManagerError::IoError(e)),
        }
    }

    pub fn clear_dirty(&self) -> Result<(), KeyManagerError> {
        let path = self.get_dirty_path();

        if path.exists() {
            fs::remove_file(path).map_err(|e| KeyManagerError::IoError(e))?;
        }

        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.get_dirty_path().exists()
    }

    fn get_dirty_path(&self) -> PathBuf {
        self.project_dir.data_dir().join(".dirty")
    }
}

fn list_dir<T: AsRef<Path>>(base: T) -> Result<Vec<PathBuf>, KeyManagerError> {
    if !base.as_ref().exists() {
        return Ok(vec![]);
    }

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
        let mut paths = list_dir(&self.path)?;

        Ok(
            paths.drain(..)
                .filter(|path| path.is_file())
                .collect()
        )
    }

    pub fn get_path(&self, key_name: &str) -> PathBuf {
        PathBuf::from(&self.path)
            .join(key_name)
    }
}
