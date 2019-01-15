pub fn append_keyboard_word_to_list_of_words(
    words: Vec<String>,
    keyboard_words: &Vec<String>,
) -> Vec<String> {
    let mut new_words: Vec<String> = vec![];
    for word in words.iter() {
        for keyboard_word in keyboard_words.iter() {
            new_words.push(append_keyboard_word_to_word(
                word.to_string(),
                keyboard_word.to_string(),
            ));
        }
    }
    new_words
}

fn append_keyboard_word_to_word(word: String, keyboard_word: String) -> String {
    let mut word_with_keyboard_word = String::from(word);
    word_with_keyboard_word.push_str(keyboard_word.as_ref());
    word_with_keyboard_word
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn should_append_keyboard_word_to_word() {
        let word = String::from("Markus");
        let keyboard_word = String::from("qwe");

        let expected_appended_word = String::from("Markusqwe");
        assert_eq!(
            expected_appended_word,
            append_keyboard_word_to_word(word, keyboard_word)
        );
    }

    #[test]
    fn should_append_keyboard_word_to_a_list_of_words() {
        let words = vec![
            String::from("Markus"),
            String::from("Mackan"),
            String::from("Mackis"),
        ];
        let keyboard_words = vec![String::from("qwe")];

        let expected_appended_word = vec![
            String::from("Markusqwe"),
            String::from("Mackanqwe"),
            String::from("Mackisqwe"),
        ];
        assert_eq!(
            expected_appended_word,
            append_keyboard_word_to_list_of_words(words, &keyboard_words)
        );
    }

    #[test]
    fn should_append_a_list_of_keyboard_words_to_a_list_of_words() {
        let words = vec![
            String::from("Markus"),
            String::from("Mackan"),
            String::from("Mackis"),
        ];
        let keyboard_words = vec![
            String::from("qwe"),
            String::from("123"),
            String::from("asd"),
        ];

        let expected_appended_word = vec![
            String::from("Markusqwe"),
            String::from("Markus123"),
            String::from("Markusasd"),
            String::from("Mackanqwe"),
            String::from("Mackan123"),
            String::from("Mackanasd"),
            String::from("Mackisqwe"),
            String::from("Mackis123"),
            String::from("Mackisasd"),
        ];
        assert_eq!(
            expected_appended_word,
            append_keyboard_word_to_list_of_words(words, &keyboard_words)
        );
    }
}