pub fn generate_words_from_keyboard_layout(
    keyboard_layout_with_strategy: String,
    word_length: usize,
) -> Vec<String> {
    let iterate_to = keyboard_layout_with_strategy.chars().count() + 1 - word_length;

    let mut generated_words: Vec<String> = Vec::new();
    for i in 0..iterate_to {
        let word = keyboard_layout_with_strategy
            .chars()
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
        assert!(generated_words
            .iter()
            .all(|item| item.chars().count() == word_length));
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
        assert!(generated_words
            .iter()
            .all(|item| item.chars().count() == word_length));
        assert!(generated_words.contains(&first_word));
        assert!(generated_words.contains(&last_word));
    }
}