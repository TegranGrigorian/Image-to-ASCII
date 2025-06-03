use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

use Image_to_ASCII::utils::{ascii_util::{ASCIIUtils}, file_util::FileUtil};

fn main() {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all("ASCII-Images").expect("Failed to create output directory");
    
    // Read all jpg files from directory
    let entries: Vec<PathBuf> = std::fs::read_dir("Images")
        .expect("Failed to read directory")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("jpg") {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    
    println!("Found {} jpg images to process", entries.len());

    // Counter for processed images - thread safe
    let counter = Arc::new(AtomicUsize::new(0));
    let total = entries.len();
    
    // Process images in parallel
    entries.par_iter().for_each(|path| {
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");
            
        // Process single image
        match process_image(path, file_stem) {
            Ok(_) => {
                // Update counter and show progress
                let processed = counter.fetch_add(1, Ordering::SeqCst) + 1;
                if processed % 100 == 0 || processed == total {
                    println!("Processed {}/{} images ({}%)", 
                        processed, 
                        total, 
                        (processed * 100) / total);
                }
            },
            Err(e) => eprintln!("Error processing {}: {}", path.display(), e),
        }
    });
    
    println!("All images processed successfully!");
}

fn process_image(path: &PathBuf, file_stem: &str) -> Result<(), String> {
    // Read and process the image
    let img = FileUtil::read_image(path.to_str().unwrap())?;
    let resized_img = FileUtil::resize_image_for_ascii(img);
    let ascii_art = ASCIIUtils::convert_image_to_ascii(Ok(resized_img))?;
    
    // Write the output file
    let output_file_path = format!("ASCII-Images/{}.txt", file_stem);
    FileUtil::write_txt_file(&output_file_path, &ascii_art)?;
    
    Ok(())
}