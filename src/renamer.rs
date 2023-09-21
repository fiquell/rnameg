use crate::{generate::generate, options::Options};
use std::{fs::rename, path::Path};

fn rename_file(old_path: &Path, parent_directory: &Path, random_name: &str) {
    let new_path = if let Some(extension) = old_path.extension() {
        let new_name = format!("{}.{}", random_name, extension.to_string_lossy());

        parent_directory.join(new_name)
    } else {
        parent_directory.join(random_name)
    };

    if let Err(err) = rename(&old_path, &new_path) {
        eprintln!("Error renaming file '{}': {}", old_path.display(), err);
    }
}

fn rename_directory(old_path: &Path, parent_directory: &Path, random_name: &str) {
    let new_path = parent_directory.join(random_name);

    if let Err(err) = rename(&old_path, &new_path) {
        eprintln!("Error renaming directory '{}': {}", old_path.display(), err);
    }
}

pub fn renamer(options: &Options) {
    for old_path in &options.args {
        let parent_directory = old_path.parent().expect("Failed to get parent directory");
        let random_name = generate(options.length);

        if old_path.is_file() {
            rename_file(old_path, parent_directory, &random_name)
        } else if old_path.is_dir() && options.directory {
            rename_directory(old_path, parent_directory, &random_name)
        } else {
            eprintln!(
                "{}: -d not specified; skipping directory '{}'",
                env!("CARGO_BIN_NAME"),
                old_path.display()
            );
        }
    }
}
