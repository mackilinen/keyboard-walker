#[derive(EnumString)]
pub enum Strategy {
    Horizontal,
    Vertical,
}

pub fn create_keyboard_layout(keyboard_layout: Vec<String>, strategy: Strategy) -> String {
    match strategy {
        Strategy::Horizontal => create_horizontal_keyboard_layout(keyboard_layout),
        Strategy::Vertical => create_vertical_keyboard_layout(keyboard_layout),
    }
}

fn get_keyboard_layout_string_capacity(keyboard_layout: &Vec<String>) -> usize {
    keyboard_layout
        .iter()
        .fold(0, |count, keyboard_layout_row| {
            count + keyboard_layout_row.len()
        })
}

fn create_horizontal_keyboard_layout(keyboard_layout: Vec<String>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(&keyboard_layout);
    let initial_string = String::with_capacity(string_capacity);

    keyboard_layout
        .iter()
        .fold(initial_string, |mut sum, item| {
            sum.push_str(item);
            sum
        })
}

fn create_vertical_keyboard_layout(keyboard_layout: Vec<String>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(&keyboard_layout);
    let mut initial_string = String::with_capacity(string_capacity);

    let string_length_max = keyboard_layout.iter().fold(0, |count, item| {
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
            let item: String = keyboard_layout[j].chars().skip(i).take(1).collect();
            initial_string.push_str(&item)
        }
    }

    initial_string
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_1_row() {
        let keyboard_layout = vec!["1234".to_string()];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);

        assert_eq!("1234".to_string(), created_keyboard_layout);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_4_rows() {
        let keyboard_layout = vec![
            "123456".to_string(),
            "qwerty".to_string(),
            "asdfgh".to_string(),
            "zxcvbn".to_string(),
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);

        assert_eq!(
            "123456qwertyasdfghzxcvbn".to_string(),
            created_keyboard_layout
        );
    }

    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_jagged_rows() {
        let keyboard_layout = vec![
            "qwerty".to_string(),
            "asd".to_string(),
            "zxcvbn".to_string(),
        ];

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
        let keyboard_layout = vec![
            "123456".to_string(),
            "qwerty".to_string(),
            "asdfgh".to_string(),
            "zxcvbn".to_string(),
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);

        assert_eq!(
            "1qaz2wsx3edc4rfv5tgb6yhn".to_string(),
            created_keyboard_layout
        );
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_jagged_rows() {
        let keyboard_layout = vec![
            "qwerty".to_string(),
            "asd".to_string(),
            "zxcvbn".to_string(),
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);

        assert_eq!("qazwsxedcrvtbyn".to_string(), created_keyboard_layout);
    }
}