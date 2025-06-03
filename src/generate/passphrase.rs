use crate::utils::clipboard;
use passforge::{Generator, PassphraseConfig, PassphraseGenerator, WordList};

pub fn passphrase() {
    let config = PassphraseConfig::new(6, "-".to_string(), WordList::Default);
    let passphrase = PassphraseGenerator::generate(&config).expect("Failed to generate passphrase");
    println!("{}", passphrase);

    clipboard::write_to_clipboard(passphrase)
}
