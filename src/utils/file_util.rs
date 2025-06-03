//read file jpgs and write txts
//these are then returned to a function to then be read by an ASCII art generator
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use image::{self, DynamicImage, imageops::FilterType};

pub struct FileUtil;

impl FileUtil {
    pub fn read_image(image_path: &str) -> Result<DynamicImage, String> {
        image::open(image_path).map_err(|e| e.to_string())
    }

    pub fn write_txt_file(output_path: &str, content: &str) -> Result<(), String> {
        let path = Path::new(output_path);
        
        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        
        // Use BufWriter for more efficient writing
        let file = File::create(output_path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(file);
        writer.write_all(content.as_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }

    //lower the image res for a 80x25 VGA text mode
    pub fn resize_image_for_ascii(image: DynamicImage) -> DynamicImage {
        // Target dimensions for VGA text mode
        let target_width = 80;
        let target_height = 25; // Using exactly 25 rows for VGA text mode

        image.resize_exact(
            target_width, 
            target_height, 
            FilterType::Lanczos3
        )
    }
}