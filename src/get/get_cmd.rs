use crate::get::iface::iface;
use crate::get::ip::ip;
use crate::get::sensors::sensors;
use crate::get::volumes::volumes;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Obtain information")]
pub struct GetCmd {
    #[command(subcommand)]
    pub get_cmd: GetSubCmd,
}

#[derive(Subcommand, Debug)]
pub enum GetSubCmd {
    #[command(about = "Get iface")]
    Iface,
    #[command(about = "Get IP information")]
    Ip,
    #[command(about = "Get sensors info")]
    Sensors,
    #[command(about = "List volumes")]
    Volumes,
}

pub fn handle_get_command(get_data: &GetCmd) {
    match &get_data.get_cmd {
        GetSubCmd::Iface => iface(),
        GetSubCmd::Ip => ip(),
        GetSubCmd::Sensors => sensors(),
        GetSubCmd::Volumes => volumes(),
    }
}
