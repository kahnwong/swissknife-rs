use crate::utils::clipboard;
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;

pub fn key() {
    let mut bytes = [0u8; 48];
    rand::thread_rng().fill_bytes(&mut bytes);

    let key = general_purpose::STANDARD.encode(bytes);
    println!("{}", key);

    clipboard::write_to_clipboard(key)
}
