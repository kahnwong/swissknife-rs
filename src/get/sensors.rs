use colored::Colorize;
use log::error;
use sysinfo::{Components, System};

pub fn sensors() {
    let mut system = System::new_all();
    system.refresh_all();

    let components = Components::new_with_refreshed_list();
    if let Some(component) = (&components).into_iter().next() {
        if let Some(temperature) = component.temperature() {
            println!("{}: {}", "Temperature".bright_green(), temperature);
        } else {
            error!("Unknown temperature: {}", component.label())
        }
    }
}
