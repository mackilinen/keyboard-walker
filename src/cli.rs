use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Generate passwords based on keyboard keys
pub struct Cli {
    /// Minimum length of generated keyboard sequence
    #[structopt(long = "min", short, default_value = "3")]
    pub min_length: usize,
    /// Maximum length of generated keyboard sequence
    #[structopt(long = "max", short = "M", default_value = "6")]
    pub max_length: usize,
    /// Strategy of generated keyboard sequence [values: All, Horizontal, Vertical]
    #[structopt(long, short, default_value = "All")]
    pub strategy: String,
    /// Starting point of which corner to start on the keyboard [values: All, TopLeft, BottomLeft, TopRight, BottomRight]
    #[structopt(long, short = "S", default_value = "All")]
    pub starting_point: String,
    /// The word list file path
    #[structopt(long, short, default_value = "", parse(from_os_str))]
    pub words_file: PathBuf,
    /// The keyboard layout file path
    #[structopt(long, short, default_value = "", parse(from_os_str))]
    pub keyboard_file: PathBuf,
    /// Concatenation order of the keyboard sequence on the word [values: Append, Prepend]
    #[structopt(long, short, default_value = "Append")]
    pub concatenation: String,
    /// Minimum depth to go on the keyboard rows/columns
    #[structopt(long, short, default_value = "0")]
    pub depth_min: usize,
    /// Maximum depth to go on the keyboard rows/columns
    #[structopt(long, short, default_value = "3")]
    pub depth_max: usize,
    /// File path to write output to (overwrite file if exist)
    #[structopt(long, short, default_value = "", parse(from_os_str))]
    pub output_file: PathBuf,
}
