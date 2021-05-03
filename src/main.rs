use structopt::StructOpt;

use crate::app::Application;
use crate::cli::{Cli, Command};

mod app;
mod cli;
mod error;

fn main() {
    let app = Application::create().unwrap();
    let args: Cli = Cli::from_args();

    let res = args.execute(&app);

    if let Err(e) = res {
        panic!("An error occurred while executing the command: {:?}", e);
    }
}
