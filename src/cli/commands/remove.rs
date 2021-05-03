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
            let res = fs::remove_file(group.get_path(key_name))
                .map_err(|e| KeyManagerError::IoError(e));

            println!("Deleted 1 key named {}", key_name);

            res
        } else {
            let key_count = group.items()?.len();

            let res = fs::remove_dir_all(group.path)
                .map_err(|e| KeyManagerError::IoError(e));

            println!("Deleted {} key(s)", key_count);

            res
        }
    }
}
