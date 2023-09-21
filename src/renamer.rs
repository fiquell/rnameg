use crate::{generate::generate, options::Options};
use std::fs::rename;

pub fn renamer(options: &Options) {
    for old_path in &options.args {
        let parent_dir = old_path.parent().expect("Failed to get parent directory");
        let random_name = generate(options.length);

        if old_path.is_file() {
            let new_path = if let Some(extension) = old_path.extension() {
                let new_name = format!("{}.{}", random_name.clone(), extension.to_string_lossy());
                parent_dir.join(new_name)
            } else {
                parent_dir.join(random_name.clone())
            };

            if let Err(err) = rename(&old_path, &new_path) {
                eprintln!("Error renaming file '{}': {}", old_path.display(), err);
            }
        } else {
            eprintln!(
                "{}: -r not specified; omitting directory '{}'",
                env!("CARGO_BIN_NAME"),
                old_path.display()
            );
        }

        if old_path.is_dir() && options.recursive {
            let new_path = parent_dir.join(random_name.clone());
            if let Err(err) = rename(&old_path, &new_path) {
                eprintln!("Error renaming directory '{}': {}", old_path.display(), err);
            }
        }
    }
}
