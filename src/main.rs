mod config;
mod shouldideploytoday;

mod get {
    pub mod get_cmd;
    pub mod iface;
    pub mod ip;
    pub mod sensors;
    pub mod system_info;
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
use crate::shouldideploytoday::shouldideploytoday;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::aot::{Shell, generate};
use config::CLAP_STYLING;
use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Completion {
        #[arg(value_enum)]
        shell: Shell,
    },
    Generate(generate_cmd::GenerateCmd),
    Get(get_cmd::GetCmd),
    #[command(about = "Should I deploy today?")]
    Shouldideploytoday {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            let bin_name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, bin_name, &mut io::stdout());
        }
        Commands::Generate(generate_data) => {
            generate_cmd::handle_generate_command(generate_data);
        }
        Commands::Get(get_data) => {
            get_cmd::handle_get_command(get_data);
        }
        Commands::Shouldideploytoday {} => shouldideploytoday(),
    }
}
