use structopt::StructOpt;
use crate::cli::Command;
use crate::error::KeyManagerError;
use crate::app::{Application, Group};

#[derive(Debug, StructOpt)]
pub struct ListCommand {
    /// If provided, only keys belonging to the named group will be displayed.
    pub group: Option<String>,
}

impl Command for ListCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        if let Some(name) = &self.group {
            display_group(app.get_group(name))?;
        } else {
            for group in app.get_groups()? {
                display_group(group)?;
            }
        }

        Ok(())
    }
}

fn display_group(group: Group) -> Result<(), KeyManagerError> {
    let items = group.items()?;

    if items.len() == 0 {
        println!("Group named {} does not exist or is empty", group.name);
    } else {
        println!("{} ({} key(s))", &group.name, items.len());

        for item in items {
            println!("  - {}", item.file_name().unwrap().to_str().unwrap());
        }

        println!();
    }

    Ok(())
}
