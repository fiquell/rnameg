use crate::{generate::generate, options::Options};
use std::fs::rename;

/// This function takes an `Options` struct as input, which contains a list of files/directories to rename,
/// desired random name length, and whether the operation should be recursive.
///
/// # Arguments
///
/// * `options` - Reference to an `Options` struct containing user-defined options.
///
/// # Errors
///
/// This function can return an error if there is a problem with renaming a file or directory.
///
/// # Examples
///
/// ```
/// use options::Options;
/// use renamer::renamer;
///
/// // Parse command-line arguments
/// let options = Options::parse_args();
///
/// // Rename the files and directories based on the provided options
/// renamer(&options);
/// ```
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
