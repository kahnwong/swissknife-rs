use crate::utils::clipboard;
use image::{ImageBuffer, Luma};
use qrcode::QrCode;
use qrcode::render::unicode;
use std::process;

fn set_url(url: &Option<String>) -> String {
    let url_parsed: String;
    match url {
        None => {
            let url_from_clipboard = clipboard::read_from_clipboard();
            if url_from_clipboard.starts_with("https://") {
                url_parsed = url_from_clipboard;
            } else {
                println!("Please specify URL");
                process::exit(1);
            }
        }
        Some(url_value) => {
            url_parsed = url_value.to_owned();
        }
    }

    url_parsed
}

fn generate_qrcode(url: String) -> (ImageBuffer<Luma<u8>, Vec<u8>>, String) {
    let code = QrCode::new(url.as_bytes()).unwrap();

    let image = code.render::<Luma<u8>>().build();
    let stdout = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    (image, stdout)
}

pub fn qrcode(url: &Option<String>) {
    let url_parsed = set_url(url);
    println!("{}", url_parsed);

    let (image, stdout) = generate_qrcode(url_parsed);
    clipboard::write_image_to_clipboard(image);

    println!("{}", stdout);
}
