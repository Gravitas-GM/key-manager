use structopt::StructOpt;

use crate::app::Application;
use crate::cli::Command;
use crate::error::KeyManagerError;

#[derive(Debug, StructOpt)]
pub struct AddCommand {
    group: String,
    public_key: String,
    key_name: Option<String>,
}

impl Command for AddCommand {
    fn execute(&self, app: &Application) -> Result<(), KeyManagerError> {
        let parsed_key = openssh_keys::PublicKey::parse(&self.public_key)
            .map_err(|_| {
                KeyManagerError::InvalidPublicKey
            })?;

        let key_name = match &self.key_name {
            Some(value) => value,
            None => parsed_key.comment.as_ref().ok_or(KeyManagerError::PublicKeyMissingComment)?,
        };

        app.get_group(&self.group).put(key_name, &self.public_key)?;
        app.mark_dirty()?;

        Ok(())
    }
}
