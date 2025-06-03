use image::{GenericImageView, DynamicImage};

pub struct ASCIIUtils {

}

impl ASCIIUtils {
    pub fn convert_image_to_ascii(image: Result<DynamicImage, String>) -> Result<String, String> {
        let img = image.map_err(|e| e.to_string())?;
        
        // Get image dimensions
        let width = img.width();
        let height = img.height();
        
        // Pre-allocate string with appropriate capacity
        let capacity = (width * height + height) as usize;
        let mut ascii_art = String::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let pixel = img.get_pixel(x, y);
                let brightness = (pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3;
                let ascii_char = ASCIIUtils::map_brightness_to_ascii(brightness);
                ascii_art.push(ascii_char);
            }
            ascii_art.push('\n');
        }

        Ok(ascii_art)
    }

    fn map_brightness_to_ascii(brightness: u32) -> char {
        // ASCII characters from dark to light
        const ASCII_CHARS: [char; 10] = ['@', '%', '#', '*', '+', '=', '-', ':', '.', ' '];
        let idx = (brightness as f32 / 255.0 * (ASCII_CHARS.len() as f32 - 1.0)).round() as usize;
        ASCII_CHARS[idx]
    }
}