mod generate;
mod options;
mod renamer;

use options::Options;
use renamer::renamer;

fn main() {
    let options = Options::parse_args();
    renamer(&options);
}
