use arboard::Clipboard;

pub fn write_to_clipboard(text: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
