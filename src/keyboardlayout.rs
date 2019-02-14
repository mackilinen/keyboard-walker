#[derive(EnumString)]
pub enum Strategy {
    Horizontal,
    Vertical,
    All,
}

pub fn create_keyboard_layout(keyboard_layout: Vec<String>, strategy: Strategy) -> Vec<String> {
    match strategy {
        Strategy::Horizontal => vec![merge_keyboard_layout_into_a_string(&keyboard_layout)],
        Strategy::Vertical => vec![merge_keyboard_layout_into_a_string(&turn_horizontal_into_vertical_keyboard_layout(keyboard_layout))],
        Strategy::All => vec![
            merge_keyboard_layout_into_a_string(&keyboard_layout),
            merge_keyboard_layout_into_a_string(&turn_horizontal_into_vertical_keyboard_layout(keyboard_layout))
        ],
    }
}

fn get_keyboard_layout_string_capacity(keyboard_layout: &Vec<String>) -> usize {
    keyboard_layout
        .iter()
        .fold(0, |count, keyboard_layout_row| {
            count + keyboard_layout_row.len()
        })
}

fn merge_keyboard_layout_into_a_string(keyboard_layout: &Vec<String>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(keyboard_layout);
    let initial_string = String::with_capacity(string_capacity);

    keyboard_layout
        .iter()
        .fold(initial_string, |mut sum, item| {
            sum.push_str(item);
            sum
        })
}

fn turn_horizontal_into_vertical_keyboard_layout(keyboard_layout: Vec<String>) -> Vec<String>{
    let mut new_layout = vec![];
    
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
        let mut initial_string = "".to_string();
        for j in 0..vector_length {
            let item: String = keyboard_layout[j].chars().skip(i).take(1).collect();
            initial_string.push_str(&item)
        }
        new_layout.push(initial_string);
    }
    
    new_layout
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_1_row() {
        let keyboard_layout = vec!["1234".to_string()];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal);

        assert_eq!("1234".to_string(), created_keyboard_layout[0]);
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
            created_keyboard_layout[0]
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

        assert_eq!("qwertyasdzxcvbn".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_1_row() {
        let keyboard_layout = vec!["1234".to_string()];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical);

        assert_eq!("1234".to_string(), created_keyboard_layout[0]);
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
            created_keyboard_layout[0]
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

        assert_eq!("qazwsxedcrvtbyn".to_string(), created_keyboard_layout[0]);
    }
    
    #[test]
    fn turn_keyboard_layout_vertical() {
        let keyboard_layout = vec![
            "qwe".to_string(),
            "asd".to_string(),
            "zxc".to_string(),
        ];
        
        let new_layout = turn_horizontal_into_vertical_keyboard_layout(keyboard_layout);

        assert_eq!(vec![
            "qaz".to_string(),
            "wsx".to_string(),
            "edc".to_string(),
        ], new_layout);
    }
}