use arboard::{Clipboard, ImageData};
use image::{ImageBuffer, Luma, Pixel, Rgba};
use std::borrow::Cow;
use std::time::Duration;

pub fn write_to_clipboard(text: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}

pub fn read_from_clipboard() -> String {
    let mut clipboard = Clipboard::new().unwrap();

    clipboard.get_text().unwrap_or_else(|_err| "".to_string())
}

pub fn write_image_to_clipboard(image: ImageBuffer<Luma<u8>, Vec<u8>>) {
    //// convert to rgba
    let (width, height) = image.dimensions();
    let mut rgba_image = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let luma_pixel = image.get_pixel(x, y);
            // For a grayscale image, set R, G, B to the grayscale value
            // and A to 255 (fully opaque)
            let gray_value = luma_pixel.channels()[0];
            let rgba_pixel = Rgba([gray_value, gray_value, gray_value, 255]);
            rgba_image.put_pixel(x, y, rgba_pixel);
        }
    }

    //// Prepare ImageData for the clipboard
    let image_data = ImageData {
        width: rgba_image.width() as usize,
        height: rgba_image.height() as usize,
        bytes: Cow::from(rgba_image.as_raw().to_vec()),
    };

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_image(image_data).unwrap();

    std::thread::sleep(Duration::from_millis(100)) // `prevent Clipboard was dropped very quickly after writing`
}
