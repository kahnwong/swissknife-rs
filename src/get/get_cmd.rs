use crate::get::iface::iface;
use crate::get::ip::ip;
use crate::get::sensors::sensors;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct GetCmd {
    #[command(subcommand)]
    pub get_cmd: GetSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum GetSubCmd {
    Iface,
    Ip,
    Sensors,
}

pub fn handle_get_command(get_data: &GetCmd) {
    match &get_data.get_cmd {
        GetSubCmd::Iface => iface(),
        GetSubCmd::Ip => ip(),
        GetSubCmd::Sensors => sensors(),
    }
}
