use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, styles = CLAP_STYLING)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Does foo things
    Foo {
        #[command(subcommand)]
        foo_command: FooCommands,
    },
    /// Does bar things
    Bar,
}

#[derive(Subcommand, Debug)]
enum FooCommands {
    /// Tests foo functionality
    Test {
        // You can add arguments specific to the 'test' subcommand here
        #[arg(short, long)]
        verbose: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Foo { foo_command } => {
            match foo_command {
                FooCommands::Test { verbose } => {
                    println!("Running foo test command. Verbose: {}", verbose);
                    // Implement your 'foo test' logic here
                }
            }
        }
        Commands::Bar => {
            println!("Running bar command");
            // Implement your 'bar' logic here
        }
    }
}
// See also `clap_cargo::style::CLAP_STYLING`
pub const CLAP_STYLING: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(clap_cargo::style::HEADER)
    .usage(clap_cargo::style::USAGE)
    .literal(clap_cargo::style::LITERAL)
    .placeholder(clap_cargo::style::PLACEHOLDER)
    .error(clap_cargo::style::ERROR)
    .valid(clap_cargo::style::VALID)
    .invalid(clap_cargo::style::INVALID);
