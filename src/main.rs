#[macro_use]
extern crate strum_macros;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;
use std::str::FromStr;
use quicli::prelude::*;
use structopt::StructOpt;
use strum;

mod appender;
mod keyboardlayout;
mod walker;

/// Generate passwords based on keyboard keys
#[derive(Debug, StructOpt)]
struct Cli {
    // Add a CLI argument `--length`/-l` that defaults to 3, and has this help text:
    /// Length of generated keyboard sequence
    #[structopt(long = "length", short = "l", default_value = "3")]
    length: usize,
    /// Strategy of generated keyboard sequence
    #[structopt(long = "strategy", short = "s", default_value = "Horizontal")]
    strategy: String,
    /// The file to read
    #[structopt(long = "words-file", short = "f", default_value = "", parse(from_os_str))]
    file: PathBuf,
    /// The file to read
    #[structopt(long = "keyboard-file", short = "k", default_value = "", parse(from_os_str))]
    keyboard_file: PathBuf,
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    
    let args = Cli::from_args();
    args.verbosity.setup_env_logger(&env!("CARGO_PKG_NAME"))?;
    
    let word_length = args.length;
    let strategy = keyboardlayout::Strategy::from_str(&args.strategy)?;
    let word_list = if args.file.as_os_str().is_empty() {
        Vec::new()
    } else {
        BufReader::new(File::open(&args.file)?)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    };
    let keyboard_layout = if args.keyboard_file.as_os_str().is_empty() {
        vec![
            "§1234567890+´".to_string(),
            "qwertyuiopå¨".to_string(),
            "asdfghjklöä'".to_string(),
            "<zxcvbnm,.-".to_string(),
        ]
    } else {
        BufReader::new(File::open(&args.keyboard_file)?)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    };

    let new_keyboard_layout = keyboardlayout::create_keyboard_layout(keyboard_layout, strategy);
    let keyboard_words = walker::generate_words_from_keyboard_layout(new_keyboard_layout, word_length);
    let new_words = appender::append_keyboard_word_to_list_of_words(word_list, &keyboard_words);
    
    for keyboard_word in keyboard_words.iter() {
        println!("{}", keyboard_word);
    }
    
    for new_word in new_words.iter() {
        println!("{}", new_word);
    }
    
    Ok(())
}