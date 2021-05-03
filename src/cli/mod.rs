use structopt::StructOpt;

use crate::app::Application;
use crate::error::KeyManagerError;
use crate::cli::commands::add::AddCommand;
use crate::cli::commands::list::ListCommand;
use crate::cli::commands::remove::RemoveCommand;
use crate::cli::commands::compile::CompileCommand;

mod commands;

pub trait Command {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError>;
}

#[derive(Debug, StructOpt)]
#[structopt(name = "SSH Keyserver Manager", about = "Manages keys to be served by a centralized keyserver.")]
pub enum Cli {
    Add(AddCommand),
    Compile(CompileCommand),
    Remove(RemoveCommand),
    List(ListCommand),
}

impl Command for Cli {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        match self {
            Self::Add(command) => command.execute(app),
            Self::Compile(command) => command.execute(app),
            Self::List(command) => command.execute(app),
            Self::Remove(command) => command.execute(app),
        }
    }
}
