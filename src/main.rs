use std::{
    collections::HashMap,
    env, fs,
    io::Result,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: <directory>");
        return;
    }

    let directory = &args[1];

    if let Err(err) = organize_file(directory) {
        eprintln!("Error organizing files: {}", err);
    } else {
        println!("Organized {} successfully", directory);
    }
}

fn map_file_to_folder() -> HashMap<&'static str, &'static str> {
    let mut file_folder_map = HashMap::new();

    file_folder_map.insert("jpeg", "Images");
    file_folder_map.insert("doc", "Documents");
    file_folder_map.insert("gif", "Images");
    file_folder_map.insert("svg", "Images");
    file_folder_map.insert("docs", "Documents");
    file_folder_map.insert("docx", "Documents");
    file_folder_map.insert("csv", "Documents");
    file_folder_map.insert("webp", "Images");
    file_folder_map.insert("webm", "Videos");

    file_folder_map
}

fn organize_file(directory: &str) -> Result<()> {
    let file_type_map_to_folder = map_file_to_folder();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if let Some(extension) = path.extension() {
            if let Some(ext_str) = extension.to_str() {
                if let Some(file_folder) = file_type_map_to_folder.get(ext_str) {
                    let target_folder_path = Path::new(directory).join(file_folder);

                    fs::create_dir_all(&target_folder_path)?;

                    let file_name = path.file_name().unwrap();

                    let target_path = target_folder_path.join(file_name);

                    let unique_path = check_unique_path(&target_path);

                    if let Err(err) = fs::rename(path, unique_path) {
                        eprintln!("Failed to move file: {}", err);
                    }
                }
            }
        }
    }

    Ok(())
}

fn check_unique_path(target_path: &PathBuf) -> PathBuf {
    let mut new_target_path = target_path.clone();
    let mut counter = 1;

    while new_target_path.exists() {
        let file_name = target_path.file_stem().unwrap().to_string_lossy();
        let file_extension = target_path
            .extension()
            .unwrap_or_default()
            .to_string_lossy();

        new_target_path =
            target_path.with_file_name(format!("{}_{}.{}", file_name, counter, file_extension));
        counter += 1;
    }
    new_target_path
}
