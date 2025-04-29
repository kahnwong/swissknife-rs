use crate::get::iface::iface;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)] // While we only need Subcommand here, Parser might be useful later if Foo itself takes direct arguments
pub struct GetCmd {
    #[command(subcommand)]
    pub foo_command: FooTestCmd,
}

#[derive(Subcommand, Debug)]
pub enum FooTestCmd {
    Iface,
}

pub fn handle_foo_command(foo_data: &GetCmd) {
    match &foo_data.foo_command {
        FooTestCmd::Iface => iface(),
    }
}
