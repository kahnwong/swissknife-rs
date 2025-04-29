use default_net;
use log::error;

pub fn iface() {
    match default_net::get_default_interface() {
        Ok(interface) => {
            println!("{}", interface.name);
        }
        Err(e) => {
            error!("Could not get default interface: {}", e)
        }
    }
}
