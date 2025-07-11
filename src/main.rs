mod config;
mod get {
    pub mod get_cmd;
    pub mod iface;
    pub mod ip;
    pub mod sensors;
    pub mod volumes;
}
mod generate {
    pub mod generate_cmd;
    pub mod key;
    pub mod passphrase;
    pub mod password;
    pub mod ssh_key;
}
mod utils {
    pub mod clipboard;
}
use crate::generate::generate_cmd;
use crate::get::get_cmd;
use clap::{Parser, Subcommand};
use config::CLAP_STYLING;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Generate(generate_cmd::GenerateCmd),
    Get(get_cmd::GetCmd),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(generate_data) => {
            generate_cmd::handle_generate_command(generate_data);
        }
        Commands::Get(get_data) => {
            get_cmd::handle_get_command(get_data);
        }
    }
}
