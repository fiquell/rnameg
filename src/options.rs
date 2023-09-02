use argparse::{ArgumentParser, List, Print, Store, StoreTrue};
use std::path::PathBuf;

/// A struct for holding command-line options and arguments.
pub struct Options {
    /// A vector of file or directory paths supplied as arguments.
    pub args: Vec<PathBuf>,
    /// The desired length of the random string.
    pub length: usize,
    /// A flag indicating whether to process directories recursively.
    pub recursive: bool,
}

impl Options {
    /// Parses command-line arguments and returns an `Options` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use options::Options;
    ///
    /// // Parse command-line arguments
    /// let options = Options::parse_args();
    ///
    /// // Access parsed options and arguments
    /// println!("Args: {:?}", options.args);
    /// println!("Length: {}", options.length);
    /// println!("Recursive: {}", options.recursive);
    /// ```
    pub fn parse_args() -> Self {
        let mut args = Vec::new();
        let mut length = 20;
        let mut recursive = false;

        {
            let mut parser = ArgumentParser::new();

            parser.set_description("Desc");
            parser
                .refer(&mut args)
                .add_argument("**/*", List, "Args")
                .required();
            parser
                .refer(&mut length)
                .add_option(&["-l", "--length"], Store, "Length");
            parser
                .refer(&mut recursive)
                .add_option(&["-r", "--recursive"], StoreTrue, "Recursive");
            parser.add_option(
                &["-v", "--version"],
                Print(env!("CARGO_PKG_VERSION").to_string()),
                "Version",
            );
            parser.parse_args_or_exit();
        }

        Options {
            args,
            length,
            recursive,
        }
    }
}
