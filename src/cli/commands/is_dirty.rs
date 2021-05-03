use structopt::StructOpt;

use crate::app::Application;
use crate::cli::Command;
use crate::error::KeyManagerError;

#[derive(Debug, StructOpt)]
pub struct IsDirtyCommand {}

impl Command for IsDirtyCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        if app.is_dirty() {
            println!("Keys are currently marked as dirty");
        } else {
            println!("Keys are NOT currently marked as dirty");
        }

        Ok(())
    }
}
