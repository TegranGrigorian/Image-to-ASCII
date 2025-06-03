//brugh this is only for converting bad apple to ascii art for a kernel that runs a vga driver 😭😭😭
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use rayon::prelude::*;

use Image_to_ASCII::utils::{ascii_util::{ASCIIUtils}, file_util::FileUtil};

fn main() {
    //make sure it exists
    std::fs::create_dir_all("ASCII-Images").expect("Failed to create output directory");
    
    let entries: Vec<PathBuf> = std::fs::read_dir("Images") // read to maek sure we have imgs
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

    //Ensure thread saftey with a counter that is shared across the threads, stop race condition
    let counter = Arc::new(AtomicUsize::new(0));
    let total = entries.len();
    
    //Process images in parallel
    entries.par_iter().for_each(|path| {
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        //code that a thread will run to process the image
        match process_image(path, file_stem) {
            Ok(_) => {
                //Update counter and show progress
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
    let img = FileUtil::read_image(path.to_str().unwrap())?;
    let resized_img = FileUtil::resize_image_for_ascii(img);
    let ascii_art = ASCIIUtils::convert_image_to_ascii(Ok(resized_img))?;
    
    //Write the output file
    let output_file_path = format!("ASCII-Images/{}.txt", file_stem);
    FileUtil::write_txt_file(&output_file_path, &ascii_art)?;
    
    Ok(())
}