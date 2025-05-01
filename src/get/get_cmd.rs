use crate::get::iface::iface;
use crate::get::ip::ip;
use crate::get::sensors::sensors;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct GetCmd {
    #[command(subcommand)]
    pub foo_iface_cmd: GetSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum GetSubCmd {
    Iface,
    Ip,
    Sensors,
}

pub fn handle_foo_command(foo_data: &GetCmd) {
    match &foo_data.foo_iface_cmd {
        GetSubCmd::Iface => iface(),
        GetSubCmd::Ip => ip(),
        GetSubCmd::Sensors => sensors(),
    }
}
