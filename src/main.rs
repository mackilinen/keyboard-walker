extern crate strum;
#[macro_use] extern crate strum_macros;

use std::env;
use std::str::FromStr;

#[derive(EnumString)]
enum Strategy {
    Horizontal,
    Vertical,
}

fn main() {
    
    if env::args().count() < 4 {
        println!("1 arg: length of the generated keyboard word");
        println!("2 arg: kayboard walk strategy (Horizontal or Vertical)");
        println!("3 arg: words to append to");
        println!("Example: 3 Horizontal firstword secondword thirdword");
        return;
    }
    
    let word_length = env::args().skip(1).take(1).collect::<String>().parse().unwrap();
    let strategy = Strategy::from_str(&env::args().skip(2).take(1).collect::<String>()).unwrap();
    let word_list = env::args().skip(3).collect();
    let swedish_keyboard = vec![
        "§1234567890+´".to_string(),
        "qwertyuiopå¨".to_string(),
        "asdfghjklöä'".to_string(),
        "<zxcvbnm,.-".to_string(),
    ];
    
    let new_keyboard_layout = create_keyboard_layout(swedish_keyboard, strategy);
    let keyboard_word = generate_words_from_keyboard_layout(new_keyboard_layout, word_length);
    let new_words = append_keyboard_word_to_list_of_words(word_list, keyboard_word);
    
    for new_word in new_words.iter() {
        println!("{}", new_word);
    }
}

fn append_keyboard_word_to_word(word: String, keyboard_word: String) -> String {
    let mut word_with_keyboard_word = String::from(word);
    word_with_keyboard_word.push_str(keyboard_word.as_ref());
    word_with_keyboard_word
}

fn append_keyboard_word_to_list_of_words(words: Vec<String>, keyboard_words: Vec<String>) -> Vec<String> {
    let mut new_words: Vec<String> = vec![];
    for word in words.iter() {
        for keyboard_word in keyboard_words.iter() {
            new_words.push(append_keyboard_word_to_word(word.to_string(), keyboard_word.to_string()));
        }
    }
    new_words
}

fn get_keyboard_layout_string_capacity(keyboard_layout: &Vec<String>) -> usize {
    keyboard_layout.iter()
        .fold(0, |count, keyboard_layout_row| count + keyboard_layout_row.len())
}

fn create_horizontal_keyboard_layout(keyboard_layout: Vec<String>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(&keyboard_layout);
    let initial_string = String::with_capacity(string_capacity);
    
    keyboard_layout.iter()
        .fold(initial_string, |mut sum, item| { sum.push_str(item); sum })
}

fn create_vertical_keyboard_layout(keyboard_layout: Vec<String>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(&keyboard_layout);
    let mut initial_string = String::with_capacity(string_capacity);
    
    let string_length_max = keyboard_layout.iter()
        .fold(0, |count, item| { 
            let new_count = item.chars().count();
            if count < new_count { 
                new_count
            } else {
                count 
            }
        });
    let vector_length = keyboard_layout.iter().count();
    
    for i in 0..string_length_max {
        for j in 0..vector_length {
            let item: String = keyboard_layout[j].chars()
                .skip(i)
                .take(1)
                .collect();
            initial_string.push_str(&item)
        }
    }
    
    initial_string
}

fn create_keyboard_layout(keyboard_layout: Vec<String>, strategy: Strategy) -> String {
    
    match strategy {
        Strategy::Horizontal => {
            create_horizontal_keyboard_layout(keyboard_layout)
        },
        Strategy::Vertical => {
            create_vertical_keyboard_layout(keyboard_layout)
        }
    }
}

fn generate_words_from_keyboard_layout(keyboard_layout_with_strategy: String, word_length: usize) -> Vec<String> {
    
    let iterate_to = keyboard_layout_with_strategy.chars().count() + 1 - word_length;

    let mut generated_words: Vec<String> = Vec::new();
    for i in 0..iterate_to {
        let word = keyboard_layout_with_strategy.chars()
            .skip(i)
            .take(word_length)
            .collect();
        generated_words.push(word);
    }
    
    generated_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_append_keyboard_word_to_word() {
        let word = String::from("Markus");
        let keyboard_word = String::from("qwe");
        
        let expected_appended_word = String::from("Markusqwe");
        assert_eq!(expected_appended_word, append_keyboard_word_to_word(word, keyboard_word));
    }
    
    #[test]
    fn should_append_keyboard_word_to_a_list_of_words() {
        let words = vec![String::from("Markus"), String::from("Mackan"), String::from("Mackis")];
        let keyboard_words = vec![String::from("qwe")];
        
        let expected_appended_word = vec![String::from("Markusqwe"), String::from("Mackanqwe"), String::from("Mackisqwe")];
        assert_eq!(expected_appended_word, append_keyboard_word_to_list_of_words(words, keyboard_words));
    }
    
    #[test]
    fn should_append_a_list_of_keyboard_words_to_a_list_of_words() {
        let words = vec![String::from("Markus"), String::from("Mackan"), String::from("Mackis")];
        let keyboard_words = vec![String::from("qwe"), String::from("123"), String::from("asd")];
        
        let expected_appended_word = vec![
            String::from("Markusqwe"), String::from("Markus123"), String::from("Markusasd"),
            String::from("Mackanqwe"), String::from("Mackan123"), String::from("Mackanasd"),
            String::from("Mackisqwe"), String::from("Mackis123"), String::from("Mackisasd")
        ];
        assert_eq!(expected_appended_word, append_keyboard_word_to_list_of_words(words, keyboard_words));
    }
    
    #[test]
    fn generate_one_word_of_3_char_from_keyboard_layout() {
        
        let keyboard_layout = "qwe".to_string();
        let word_length = 3;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, word_length);
        
        assert_eq!(word_length, generated_words[0].chars().count());
        assert_eq!(vec!["qwe".to_string()], generated_words);
    }
    
    #[test]
    fn generate_one_word_of_6_char_from_keyboard_layout() {
        
        let keyboard_layout = "qwerty".to_string();
        let word_length = 6;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, word_length);
        
        assert_eq!(word_length, generated_words[0].chars().count());
        assert_eq!(vec!["qwerty".to_string()], generated_words);
    }
    
    #[test]
    fn generate_3_words_of_3_char_from_keyboard_layout() {
        
        let keyboard_layout = "qwert".to_string();
        let word_length = 3;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, word_length);
        
        let expected_words = vec!["qwe".to_string(), "wer".to_string(), "ert".to_string()];
        assert!(generated_words.iter().all(|item| item.chars().count() == word_length));
        assert_eq!(expected_words, generated_words);
    }
    
    #[test]
    fn generate_multiple_words_of_3_char_from_keyboard_layout() {
        
        let keyboard_layout = "qwertyasdfghzxcvbn".to_string();
        let word_length = 3;
        
        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, word_length);
        
        let first_word = "qwe".to_string();
        let last_word = "vbn".to_string();
        assert!(generated_words.iter().count() > 2);
        assert!(generated_words.iter().all(|item| item.chars().count() == word_length));
        assert!(generated_words.contains(&first_word));
        assert!(generated_words.contains(&last_word));
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_1_row() {
        
        let keyboard_layout = vec!["1234".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);
        
        assert_eq!("1234".to_string(), created_keyboard_layout);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_4_rows() {
        
        let keyboard_layout = vec!["123456".to_string(), "qwerty".to_string(), "asdfgh".to_string(), "zxcvbn".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);
        
        assert_eq!("123456qwertyasdfghzxcvbn".to_string(), created_keyboard_layout);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_jagged_rows() {
        
        let keyboard_layout = vec!["qwerty".to_string(), "asd".to_string(), "zxcvbn".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);
        
        assert_eq!("qwertyasdzxcvbn".to_string(), created_keyboard_layout);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_1_row() {
        
        let keyboard_layout = vec!["1234".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);
        
        assert_eq!("1234".to_string(), created_keyboard_layout);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_4_rows() {
        
        let keyboard_layout = vec!["123456".to_string(), "qwerty".to_string(), "asdfgh".to_string(), "zxcvbn".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);
        
        assert_eq!("1qaz2wsx3edc4rfv5tgb6yhn".to_string(), created_keyboard_layout);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_jagged_rows() {
        
        let keyboard_layout = vec!["qwerty".to_string(), "asd".to_string(), "zxcvbn".to_string()];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);
        
        assert_eq!("qazwsxedcrvtbyn".to_string(), created_keyboard_layout);
    }
}