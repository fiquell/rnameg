use argparse::{ArgumentParser, List, Print, Store, StoreTrue};
use std::path::PathBuf;

pub struct Options {
    pub args: Vec<PathBuf>,
    pub length: usize,
    pub directory: bool,
}

impl Options {
    pub fn parse_args() -> Self {
        let mut args = Vec::new();
        let mut length = 20;
        let mut directory = false;

        {
            let mut parser = ArgumentParser::new();

            parser.set_description("Command-line utility for renaming a file or directory");
            parser
                .refer(&mut args)
                .add_argument(
                    "SOURCES",
                    List,
                    "One or more file or directory paths to process",
                )
                .required();
            parser.refer(&mut length).add_option(
                &["-l", "--length"],
                Store,
                "Set a random string length (default: 20)",
            );
            parser.refer(&mut directory).add_option(
                &["-d", "--directory"],
                StoreTrue,
                "Enable directory renaming",
            );
            parser.add_option(
                &["-v", "--version"],
                Print(env!("CARGO_PKG_VERSION").to_string()),
                "Show this version message and exit",
            );
            parser.parse_args_or_exit();
        }

        Options {
            args,
            length,
            directory,
        }
    }
}
