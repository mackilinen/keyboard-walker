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
    /// The word list file path
    #[structopt(long = "words-file", short, default_value = "", parse(from_os_str))]
    pub words_file: PathBuf,
    /// The keyboard layout file path
    #[structopt(long = "keyboard-file", short, default_value = "", parse(from_os_str))]
    pub keyboard_file: PathBuf,
    /// Concatenation order of the keyboard sequence on the word [values: Append, Prepend]
    #[structopt(long, short, default_value = "Append")]
    pub concatenation: String,
    /// How deep to go on the keyboard rows
    #[structopt(long, short, default_value = "0")]
    pub depths: usize,
}
