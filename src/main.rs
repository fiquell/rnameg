use argparse::{ArgumentParser, List, Store};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{char, fs::rename, path::PathBuf};

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
}

fn main() {
    let mut options = Options {
        files: Vec::new(),
        length: 20,
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
        parser.parse_args_or_exit();
    }

    for old_path in &options.files {
        if old_path.is_file() {
            let parent_dir = old_path.parent().expect("Failed to get parent directory");
            // println!("old parent_dir: {}", parent_dir.display());

            // let mut filenames = parent_dir.to_path_buf();
            let random_name = generate_random_names(options.length);
            // println!("random_name: {}", random_name);
            // let ext_names = current_name.extension().and_then(|ext| ext.to_str());

            let new_path = if let Some(extension) = old_path.extension() {
                let new_name = format!("{}.{}", random_name, extension.to_string_lossy());
                // println!("extension: {}", extension.to_string_lossy());
                // println!("new_name: {}", new_name);
                parent_dir.join(new_name)
            } else {
                parent_dir.join(random_name)
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
        }
    }
}
