use crate::generate::key::key;
use crate::generate::passphrase::passphrase;
use crate::generate::password::password;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct GenerateCmd {
    #[command(subcommand)]
    pub generate_cmd: GenerateSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubCmd {
    Key,
    Passphrase,
    Password,
}

pub fn handle_generate_command(get_data: &GenerateCmd) {
    match &get_data.generate_cmd {
        GenerateSubCmd::Key => key(),
        GenerateSubCmd::Passphrase => passphrase(),
        GenerateSubCmd::Password => password(),
    }
}
