pub fn generate_words_from_keyboard_layout_with_min_max(
    keyboard_layout_with_strategy: String,
    min_word_length: usize,
    max_word_length: usize,
) -> Vec<String> {
    let mut generated_words: Vec<String> = Vec::new();
    let keyboard_layout_length = keyboard_layout_with_strategy.chars().count();
    
    let iterate_to_length = get_iteration_length(max_word_length, keyboard_layout_length);
    let index_start = 0;

    for length in min_word_length..=iterate_to_length {
        let iterate_to = keyboard_layout_length - length;
        for i in index_start..=iterate_to {
            let word = get_word_from_keyboard_layout(&keyboard_layout_with_strategy, i, length);
            generated_words.push(word);
        }
    }

    generated_words
}

fn get_word_from_keyboard_layout(
    keyboard_layout: &str,
    index: usize,
    number_of_chars: usize,
) -> String {
    keyboard_layout
        .chars()
        .skip(index)
        .take(number_of_chars)
        .collect()
}

fn get_iteration_length(max_word_length: usize, keyboard_layout_length: usize) -> usize {
    if max_word_length < keyboard_layout_length {
        max_word_length
    } else {
        keyboard_layout_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_words_from_keyboard_layout(
        keyboard_layout_with_strategy: String,
        word_length: usize,
    ) -> Vec<String> {
        generate_words_from_keyboard_layout_with_min_max(
            keyboard_layout_with_strategy,
            word_length,
            word_length,
        )
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

    #[test]
    fn generate_multiple_words_of_min_3_char_and_max_4_char_from_keyboard_layout() {
        let keyboard_layout = "qwer".to_string();
        let min_word_length = 3;
        let max_word_length = 4;

        let generated_words = generate_words_from_keyboard_layout_with_min_max(
            keyboard_layout,
            min_word_length,
            max_word_length,
        );

        let first_word = "qwe".to_string();
        let last_word = "qwer".to_string();

        assert!(generated_words.iter().count() == 3);
        assert!(generated_words.contains(&first_word));
        assert!(generated_words.contains(&last_word));
    }

    #[test]
    fn dont_panic_when_max_length_is_bigger_then_keyboard_layout_length() {
        let keyboard_layout = "qwer".to_string();
        let min_word_length = 3;
        let max_word_length = 6;

        let generated_words = generate_words_from_keyboard_layout_with_min_max(
            keyboard_layout,
            min_word_length,
            max_word_length,
        );

        let first_word = "qwe".to_string();
        let last_word = "qwer".to_string();

        assert!(generated_words.iter().count() == 3);
        assert!(generated_words.contains(&first_word));
        assert!(generated_words.contains(&last_word));
    }
    
    #[test]
    fn dont_generate_word_from_empty_keyboard_layout() {
        let keyboard_layout = "".to_string();
        let word_length = 3;

        let generated_words = generate_words_from_keyboard_layout(keyboard_layout, word_length);

        assert_eq!(0, generated_words.len());
    }
}