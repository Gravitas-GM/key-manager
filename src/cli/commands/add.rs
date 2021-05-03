use structopt::StructOpt;

use crate::app::Application;
use crate::cli::Command;
use crate::error::KeyManagerError;

#[derive(Debug, StructOpt)]
pub struct AddCommand {
    /// The group name to store the new key under.
    group: String,

    /// The new public key.
    public_key: String,

    /// The public key's name; if not provided, the public key's comment value will be used instead.
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
