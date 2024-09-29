use colored::*;
use crossterm::execute;
use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{self};
use std::path::{Path, PathBuf};
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    // Enable ANSI support for Windows terminals
    if cfg!(target_os = "windows") {
        let _ = execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        );
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: file_organizer <directory>");
        return;
    }

    let directory = &args[1];

    // Notify user that files are being organized
    println!("{}", "Organizing files...".blue());

    if let Err(e) = organize_files(directory) {
        eprintln!("{}", e.to_string().red()); // Convert the error to string before coloring
    } else {
        println!("{} {}", directory, "organized successfully!".green());
    }
}

// Ensure the rest of your code remains unchanged

fn get_file_type_mapping() -> HashMap<&'static str, &'static str> {
    let mut file_types = HashMap::new();

    // Map file extensions to folder names
    file_types.insert("jpg", "Images");
    file_types.insert("png", "Images");
    file_types.insert("HEIC", "Images");
    file_types.insert("txt", "Documents");
    file_types.insert("pdf", "Documents");
    file_types.insert("xlsx", "Documents");
    file_types.insert("json", "JSON");
    file_types.insert("mp4", "Videos");

    file_types
}

fn organize_files(directory: &str) -> std::io::Result<()> {
    let file_types = get_file_type_mapping();

    // Collect all entries from WalkDir first, then process them in parallel
    let entries: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok()) // Filter out errors in directory traversal
        .collect();

    // Process each entry in parallel
    entries.par_iter().for_each(|entry| {
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            return;
        }

        // Process files based on extension
        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if let Some(folder_name) = file_types.get(ext_str) {
                    let target_dir = Path::new(directory).join(folder_name);

                    // Create the target directory if it doesn't exist
                    if let Err(e) = fs::create_dir_all(&target_dir) {
                        eprintln!("Failed to create directory {:?}: {}", target_dir, e);
                        return;
                    }

                    let file_name = path.file_name().unwrap();
                    let target_path = target_dir.join(file_name);

                    let unique_target_path = get_unique_target_path(&target_path);

                    // Move the file to the target folder
                    if let Err(e) = fs::rename(path, &unique_target_path) {
                        eprintln!(
                            "Failed to move file {:?} to {:?}: {}",
                            path, unique_target_path, e
                        );
                    }
                }
            }
        }
    });

    Ok(())
}

// Function to handle file name collisions by appending a counter to the file name
fn get_unique_target_path(target_path: &PathBuf) -> PathBuf {
    let mut new_target_path = target_path.clone();
    let mut counter = 1;

    // Continue checking if the file exists and append a counter if necessary
    while new_target_path.exists() {
        let file_name = target_path.file_stem().unwrap().to_string_lossy();
        let extension = target_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy();
        new_target_path =
            target_path.with_file_name(format!("{}_{}.{}", file_name, counter, extension));
        counter += 1;
    }

    new_target_path
}
