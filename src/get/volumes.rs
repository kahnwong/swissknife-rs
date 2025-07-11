use bytesize::ByteSize;
use colored::Colorize;
use sysinfo::Disks;
use tabled::{builder::Builder, settings::Style};

pub fn volumes() {
    // init
    let mut b = Builder::from_iter([[
        "MOUNTED ON",
        "SIZE",
        "USED",
        "AVAIL",
        "USE%",
        "TYPE",
        "FILESYSTEM",
    ]]);

    // get disks info
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if !disk
            .mount_point()
            .to_str()
            .expect("Can't cast mount_point() to string")
            .contains("/var/snap")
        {
            let mounted_on = format!("{}", disk.mount_point().to_string_lossy().bright_blue());
            let size = ByteSize::b(disk.total_space()).to_string();
            let used = ByteSize::b(disk.total_space() - disk.available_space()).to_string();

            // disk available
            let avail_raw = ByteSize::b(disk.available_space());
            let avail: String;
            //// if free space less than 50 GB
            if avail_raw < ByteSize::gb(50) {
                avail = avail_raw.to_string().red().to_string();
            }
            //// if free space less than 100 GB
            else if avail_raw < ByteSize::gb(100) {
                avail = avail_raw.to_string().yellow().to_string();
            } else {
                avail = avail_raw.to_string().bright_green().to_string();
            }

            // used pct
            let used_pct_raw = (((disk.total_space() - disk.available_space()) as f64
                / disk.total_space() as f64)
                * 100.0)
                .round() as u64;
            let used_pct: String;
            if used_pct_raw > 80 {
                used_pct = format!("{}%", used_pct_raw).to_string().red().to_string();
            } else if used_pct_raw > 70 {
                used_pct = format!("{}%", used_pct_raw)
                    .to_string()
                    .yellow()
                    .to_string();
            } else {
                used_pct = format!("{}%", used_pct_raw)
                    .to_string()
                    .bright_green()
                    .to_string();
            }

            let disk_type = format!("{}", disk.file_system().to_string_lossy().bright_magenta()); // naming according to `duf`
            let filesystem = format!("{}", disk.name().to_string_lossy().bright_magenta()); // naming according to `duf`

            b.push_record([
                mounted_on, size, used, avail, used_pct, disk_type, filesystem,
            ]);
        }
    }

    // render table
    let mut table = b.build();
    table.with(Style::rounded());

    println!("{}", table);
}
