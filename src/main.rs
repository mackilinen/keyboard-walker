#[macro_use]
extern crate strum_macros;

use failure;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;
use structopt::StructOpt;
use strum;

mod cli;
mod concatenator;
mod keyboardlayout;
mod walker;

fn main() -> Result<(), failure::Error> {
    let args = cli::Cli::from_args();
    
    let depths = args.depths;
    let min_word_length = args.min_length;
    let max_word_length = args.max_length;
    let strategy = keyboardlayout::Strategy::from_str(&args.strategy)?;
    let concatenation = concatenator::ConcatenationOrder::from_str(&args.concatenation)?;
    let word_list = if args.words_file.as_os_str().is_empty() {
        Vec::new()
    } else {
        BufReader::new(File::open(&args.words_file)?)
            .lines()
            .map(|l| l.unwrap())
            .collect()
    };
    let keyboard_layout = if args.keyboard_file.as_os_str().is_empty() {
        vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),"7".to_string(),"8".to_string(),"9".to_string(),"0".to_string(),"+".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),"u".to_string(),"i".to_string(),"o".to_string(),"p".to_string(),"å".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),"j".to_string(),"k".to_string(),"l".to_string(),"ö".to_string(),"ä".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),"m".to_string(),],
        ]
    } else {
        serde_json::from_reader(BufReader::new(File::open(&args.keyboard_file)?))?
    };

    let keyboard_layouts = keyboardlayout::create_keyboard_layout(keyboard_layout, strategy, depths);

    let mut keyboard_words = vec![];
    for keyboard_layout in keyboard_layouts {
        keyboard_words.extend(walker::generate_words_from_keyboard_layout_with_min_max(
            keyboard_layout,
            min_word_length,
            max_word_length,
        ));
    }
    let new_words = concatenator::concatenate_keyboard_word_to_list_of_words(
        word_list,
        &keyboard_words,
        concatenation,
    );

    let mut output_words = Vec::new();
    output_words.extend(keyboard_words);
    output_words.extend(new_words);

    {
        let stdout = io::stdout();
        let mut buf = BufWriter::new(stdout.lock());

        for line in output_words.iter() {
            writeln!(buf, "{}", line)?;
        }
    }

    Ok(())
}
