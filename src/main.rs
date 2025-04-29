mod config;
use clap::{Parser, Subcommand};
use config::CLAP_STYLING;
mod get {
    pub mod get_cmd;
    pub mod iface;
}
use crate::get::{get_cmd, iface};
use iface::FOOCONST;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Foo(get_cmd::FooCmd),
    Bar,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Foo(foo_data) => {
            get_cmd::handle_foo_command(foo_data);
        }
        Commands::Bar => {
            println!("Running bar command");
            // Implement your 'bar' logic here
        }
    }

    println!("{}", FOOCONST);
}
