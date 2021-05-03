use std::fs;
use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::app::Application;
use crate::cli::Command;
use crate::error::KeyManagerError;

#[derive(Debug, StructOpt)]
pub struct CompileCommand {
    /// Output will be written to this path.
    #[structopt(parse(from_os_str))]
    out_file: PathBuf,

    /// Forces output of the compiled keys file.
    ///
    /// By default, the keys file will only be regenerated if a key has been added or removed since
    /// `compile` was last run. This flag can be used to override this behavior and force the
    /// keys file to be regenerated.
    #[structopt(long, short)]
    force: bool,
}

impl Command for CompileCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        if !self.force && !app.is_dirty() {
            return Ok(());
        }

        let groups = app.get_groups()?;

        if groups.is_empty() {
            println!("No keys to compile");
        } else {
            let mut file = fs::File::create(&self.out_file)
                .map_err(|e| KeyManagerError::IoError(e))?;

            for group in groups {
                for item in group.items()? {
                    let mut key = fs::read(item)
                        .map_err(|e| KeyManagerError::IoError(e))?;

                    key.push(b'\n');

                    file.write(key.as_slice()).map_err(|e| KeyManagerError::IoError(e))?;
                }
            }
        }

        app.clear_dirty()?;

        Ok(())
    }
}
