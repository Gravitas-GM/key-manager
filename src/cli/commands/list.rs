use structopt::StructOpt;
use crate::cli::Command;
use crate::error::KeyManagerError;
use crate::app::{Application, Group};

#[derive(Debug, StructOpt)]
pub struct ListCommand {
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

    println!("{} ({} key{})", &group.name, items.len(), match items.len() {
        1 => "",
        _ => "s",
    });

    for item in items {
        println!("  - {}", item.file_name().unwrap().to_str().unwrap());
    }

    println!();

    Ok(())
}
