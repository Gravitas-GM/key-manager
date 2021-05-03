use std::fs;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::app::Application;
use crate::cli::Command;
use crate::error::KeyManagerError;

#[derive(Debug, StructOpt)]
pub struct CompileCommand {
    #[structopt(parse(from_os_str))]
    out_file: PathBuf,
}

impl Command for CompileCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        let mut file = fs::File::create(&self.out_file)
            .map_err(|e| KeyManagerError::IoError(e))?;

        for group in app.get_groups()? {
            for item in group.items()? {
                let mut key = fs::read(item).map_err(|e| KeyManagerError::IoError(e))?;
                key.push(b'\n');

                file.write(key.as_slice()).map_err(|e| KeyManagerError::IoError(e))?;
            }
        }

        Ok(())
    }
}
