use crate::utils::clipboard;
use passforge::{Generator, Length, PasswordConfig, PasswordGenerator};

pub fn password() {
    let config = PasswordConfig::new(Length::Single(32), true, true, false);
    let password = PasswordGenerator::generate(&config).expect("Failed to generate password");
    println!("{}", password);

    clipboard::write_to_clipboard(password)
}
