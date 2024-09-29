# File Organizer

This is a lightweight Rust program designed to help you organize files in a directory by categorizing them into folders based on file extensions (e.g., images, documents, videos). It ensures files are moved to their appropriate folders without overwriting existing files by adding a unique identifier to files with the same name.

## How It Works

The program recursively scans the specified directory and organizes files into predefined folders according to their file extensions. If a file with the same name already exists in the target folder, the program appends a unique number to the filename to avoid any conflict.

### Supported File Types

The file extensions are mapped to folders through a configurable mapping within the `file_folder_map` function. You can easily customize the mapping by modifying this function in the source code.

Out-of-the-box, the following file types are supported:

- **Images**: `jpeg`,`jpg`,`png`, `HEIC`, `gif`, `svg`, `webp`
- **Documents**: `doc`, `txt`, `pdf`, `xlsx`, `docx`, `csv`, `docs`
- **Videos**: `webm`, `mp4`
- **JSON**: `json`

You can add support for additional file types by simply updating the `file_folder_map` in the code.

## Usage

### Prerequisites

To run this program, you need:

- [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Cloning the Repository

1. Open a terminal and run the following commands to clone the repository and navigate to the project folder:

   ```bash
   git clone https://github.com/your-username/file-organizer.git
   cd file-organizer
   ```

2. Run the program using Cargo, passing the directory you want to organize as an argument:

   ```bash
   cargo run -- <directory-path>
   ```

   Replace `<directory-path>` with the actual path to the folder you want to organize.

   For example:

   ```bash
   cargo run -- C:/Users/admin/Downloads
   ```

   This will organize the `Downloads` folder by sorting files into subfolders like `Images`, `Documents`, `Videos`, and `JSON`.

### Customization

You can easily customize the mapping of file extensions to folders by editing the `map_file_to_folder()` function in the source code. To add support for a new file type, just insert a new line like this:

```rust
file_folder_map.insert("pdf", "Documents");
```

This will move all `.pdf` files into the `Documents` folder.

## Features

- **File organization by extension**: Automatically organizes files into predefined folders based on their extensions.
- **Conflict handling**: If a file with the same name exists in the target folder, the program appends a unique identifier to the new file's name to prevent overwriting.
- **Customizable mapping**: Easily modify the folder mappings to support new file types or change the existing ones.
- **Cross-platform**: Works on Linux, macOS, and Windows.

## Contributing

Contributions are welcome! If you have suggestions for new features, optimizations, or bug fixes, feel free to open an issue or submit a pull request.
