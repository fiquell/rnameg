use argparse::{ArgumentParser, List, Store, StoreTrue};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{char, env, fs::rename, path::PathBuf};

fn generate_random_names(length: usize) -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

struct Options {
    files: Vec<PathBuf>,
    length: usize,
    recursive: bool,
}

fn main() {
    let mut options = Options {
        files: Vec::new(),
        length: 20,
        recursive: false,
    };

    {
        let mut parser = ArgumentParser::new();

        parser.set_description(
            "Command-line utility for renaming files with a random name generator",
        );
        parser
            .refer(&mut options.files)
            .add_argument("<files>", List, "Files")
            .required();
        parser
            .refer(&mut options.length)
            .add_option(&["-l", "--length"], Store, "Length")
            .metavar("<length>");
        parser
            .refer(&mut options.recursive)
            .add_option(&["-r", "--recursive"], StoreTrue, "Recursive")
            .metavar("<recursive>");
        parser.parse_args_or_exit();
    }

    for old_path in &options.files {
        let parent_dir = old_path.parent().expect("Failed to get parent directory");
        // println!("parent_dir: {}", parent_dir.display());
        let random_name = generate_random_names(options.length);
        // println!("random_name: {}", random_name);

        if old_path.is_file() {
            // let parent_dir = old_path.parent().expect("Failed to get parent directory");
            // println!("old parent_dir: {}", parent_dir.display());

            // let mut filenames = parent_dir.to_path_buf();
            // let random_name = generate_random_names(options.length);
            // println!("random_name: {}", random_name);
            // let ext_names = current_name.extension().and_then(|ext| ext.to_str());

            let new_path = if let Some(extension) = old_path.extension() {
                let new_name = format!("{}.{}", random_name.clone(), extension.to_string_lossy());
                // println!("extension: {}", extension.to_string_lossy());
                // println!("new_name: {}", new_name);
                parent_dir.join(new_name)
            } else {
                parent_dir.join(random_name.clone())
            };

            // println!("new parent_dir: {}", parent_dir.display());
            // println!("new_path: {}", new_path.display());
            // println!("--------------------------")

            if let Err(err) = rename(&old_path, &new_path) {
                eprintln!("Error renaming file '{}': {}", old_path.display(), err);
            }

            // if let Some(ext) = ext_names {
            //     filenames.push(format!("{}.{}", random_filenames, ext))
            // } else {
            //     filenames.push(&random_filenames)
            // }

            // rename(&current_name, &filenames);
        } else {
            eprintln!(
                "{}: -r not specified; omitting directory '{}'",
                env!("CARGO_BIN_NAME"),
                old_path.display()
            );
        }

        if old_path.is_dir() && options.recursive {
            let new_path = parent_dir.join(random_name.clone());
            // println!("new_path: {}", new_path.display());

            if let Err(err) = rename(&old_path, &new_path) {
                eprintln!("Error renaming directory '{}': {}", old_path.display(), err);
            }
        }
    }
}
