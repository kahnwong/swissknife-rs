use battery;
use bytesize::ByteSize;
use colored::Colorize;
use sysinfo::{Disks, System};
use users::{get_current_uid, get_user_by_uid};

pub fn system_info() {
    // init
    let mut sys = System::new_all();
    sys.refresh_all();

    // user@hostname
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!(
        "{}@{}",
        user.name().to_string_lossy().bright_green(),
        System::host_name().unwrap().to_string().bright_green()
    );

    // line break
    let line_break = "-".repeat(
        user.name().to_string_lossy().len() + System::host_name().unwrap().to_string().len() + 1,
    );
    println!("{}", line_break);

    // OS
    println!(
        "{}: {} {}",
        "OS".bright_green(),
        System::name().unwrap(),
        System::os_version().unwrap()
    );

    // CPU
    println!("{}: {}", "CPU".bright_green(), sys.cpus()[0].brand());

    // memory
    let memory_used = format!(
        "{}%",
        (sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0) as u64
    );
    println!(
        "{}: {} / {} ({})",
        "Memory".bright_green(),
        ByteSize::b(sys.used_memory()),
        ByteSize::b(sys.total_memory()),
        memory_used.to_string().bright_blue(),
    );

    // disk
    let mut disk_used: String = "".to_string();
    let mut disk_total: String = "".to_string();
    let mut disk_used_pct: u64 = 0;
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if disk
            .mount_point()
            .to_str()
            .expect("Can't cast mount_point() to string")
            == "/"
        {
            disk_used = ByteSize::b(disk.total_space() - disk.available_space()).to_string();
            disk_total = ByteSize::b(disk.total_space()).to_string();
            disk_used_pct = (((disk.total_space() - disk.available_space()) as f64
                / disk.total_space() as f64)
                * 100.0)
                .round() as u64;
        }
    }
    let disk_used_pct_str = format!("{}%", disk_used_pct).bright_blue();

    println!(
        "{}: {} / {} ({})",
        "Disk".bright_green(),
        disk_used,
        disk_total,
        disk_used_pct_str
    );

    // battery
    let manager = battery::Manager::new().unwrap();
    for maybe_battery in manager.batteries().unwrap() {
        let battery = maybe_battery.unwrap();

        //// battery pct
        let battery_pct = (battery.state_of_charge().value as f64 * 100.00) as u64;
        let battery_pct_str: String;
        if battery_pct > 80 {
            battery_pct_str = format!("{}%", battery_pct)
                .to_string()
                .bright_green()
                .to_string();
        } else if battery_pct > 70 {
            battery_pct_str = format!("{}%", battery_pct).to_string().yellow().to_string();
        } else {
            battery_pct_str = format!("{}%", battery_pct).to_string().red().to_string();
        }

        //// battery health
        let battery_health = (battery.state_of_health().value as f64 * 100.00) as u64;
        let battery_health_str: String;
        if battery_health > 90 {
            battery_health_str = format!("{}%", battery_health)
                .to_string()
                .bright_green()
                .to_string();
        } else if battery_health > 80 {
            battery_health_str = format!("{}%", battery_health)
                .to_string()
                .yellow()
                .to_string();
        } else {
            battery_health_str = format!("{}%", battery_health).to_string().red().to_string();
        }

        println!(
            "{}: {} (Health: {} - {} Cycles)",
            "Battery".bright_green(),
            battery_pct_str,
            battery_health_str,
            battery.cycle_count().unwrap()
        );
    }
}
