use crate::generate::key::key;
use crate::generate::passphrase::passphrase;
use crate::generate::password::password;
use crate::generate::qrcode::qrcode;
use crate::generate::ssh_key::ssh_key;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Generate stuff")]
pub struct GenerateCmd {
    #[command(subcommand)]
    pub generate_cmd: GenerateSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum GenerateSubCmd {
    #[command(about = "Generate key")]
    Key,
    #[command(about = "Generate passphrase")]
    Passphrase,
    #[command(about = "Generate password")]
    Password,
    #[command(about = "Generate QR code")]
    Qrcode { url: Option<String> },

    #[command(name = "ssh-key", about = "Create SSH key")]
    SshKey { name: String },
}

pub fn handle_generate_command(get_data: &GenerateCmd) {
    match &get_data.generate_cmd {
        GenerateSubCmd::Key => key(),
        GenerateSubCmd::Passphrase => passphrase(),
        GenerateSubCmd::Password => password(),
        GenerateSubCmd::Qrcode { url } => qrcode(url),
        GenerateSubCmd::SshKey { name } => ssh_key(name).expect("Error generating ssh key"),
    }
}
