use structopt::StructOpt;
use crate::cli::Command;
use crate::error::KeyManagerError;
use crate::app::Application;
use std::fs;

#[derive(Debug, StructOpt)]
pub struct RemoveCommand {
    group: String,
    key_name: Option<String>,
}

impl Command for RemoveCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        let group = app.get_group(&self.group);

        if let Some(key_name) = &self.key_name {
            let path = group.get_path(key_name);

            if !path.exists() {
                println!("Specified key does not exist");

                Ok(())
            } else {
                let res = fs::remove_file(group.get_path(key_name))
                    .map_err(|e| KeyManagerError::IoError(e));

                if res.is_ok() {
                    app.mark_dirty()?;

                    println!("Deleted 1 key named {}", key_name);
                }

                res
            }
        } else {
            let key_count = group.items()?.len();

            if key_count == 0 {
                println!("Group named {} does not exist or is empty", group.name);

                Ok(())
            } else {
                let res = fs::remove_dir_all(group.path)
                    .map_err(|e| KeyManagerError::IoError(e));

                if res.is_ok() {
                    app.mark_dirty()?;

                    println!("Deleted {} key(s)", key_count);
                }

                res
            }
        }
    }
}
