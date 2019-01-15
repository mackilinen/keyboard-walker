#[macro_use]
extern crate strum_macros;

use std::env;
use std::str::FromStr;
use strum;

mod appender;
mod keyboardlayout;
mod walker;

fn main() {
    if env::args().count() < 4 {
        println!();
        println!("USAGE:");
        println!("  keyboard-walker <1arg> <2arg> <3arg...Narg>");
        println!();
        println!("Arguments:");
        println!("  1arg: length of the generated keyboard word");
        println!("  2arg: kayboard walk strategy (Horizontal or Vertical)");
        println!("  3arg: words to append to");
        println!();
        println!("EXAMPLE:");
        println!("  keyboard-walker 3 Horizontal firstword secondword thirdword");
        return;
    }

    let word_length = env::args()
        .skip(1)
        .take(1)
        .collect::<String>()
        .parse()
        .unwrap();
    let strategy = keyboardlayout::Strategy::from_str(&env::args().skip(2).take(1).collect::<String>()).unwrap();
    let word_list = env::args().skip(3).collect();
    let swedish_keyboard = vec![
        "§1234567890+´".to_string(),
        "qwertyuiopå¨".to_string(),
        "asdfghjklöä'".to_string(),
        "<zxcvbnm,.-".to_string(),
    ];

    let new_keyboard_layout = keyboardlayout::create_keyboard_layout(swedish_keyboard, strategy);
    let keyboard_word = walker::generate_words_from_keyboard_layout(new_keyboard_layout, word_length);
    let new_words = appender::append_keyboard_word_to_list_of_words(word_list, keyboard_word);

    for new_word in new_words.iter() {
        println!("{}", new_word);
    }
}