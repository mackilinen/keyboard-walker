#[derive(EnumString)]
pub enum Strategy {
    Horizontal,
    Vertical,
    All,
}

pub enum StartingPoint {
    TopLeft,
    BottomLeft,
    TopRight,
    BottomRight,
    All,
}

pub fn create_keyboard_layout(
    keyboard_layout: Vec<Vec<String>>,
    strategy: Strategy,
    depths: usize,
    starting_point: StartingPoint,
) -> Vec<String> {
    let keyboard_layouts = apply_strategy_and_depths(keyboard_layout, strategy, depths, starting_point);
    keyboard_layouts.iter()
        .map(|layout| merge_keyboard_layout_into_a_string(layout))
        .collect()
}

fn apply_strategy_and_depths(
    keyboard_layout: Vec<Vec<String>>,
    strategy: Strategy,
    depths: usize,
    starting_point: StartingPoint,
) -> Vec<Vec<Vec<String>>> {
    
    let keyboard_layouts_with_starting_point = apply_starting_point_to_keyboard_layout(keyboard_layout, starting_point);
    
    let mut keyboard_layouts_with_strategy = vec![];
    for keyboard_layout_with_starting_point in keyboard_layouts_with_starting_point {
        let keyboard_layout_with_strategy = apply_strategy_to_keyboard_layout(keyboard_layout_with_starting_point, &strategy);
        keyboard_layouts_with_strategy.extend(keyboard_layout_with_strategy);
    }
    
    let keyboard_layouts_with_depth = apply_depths_to_keyboard_layouts(&keyboard_layouts_with_strategy, 0, depths);
    
    let mut keyboard_layouts = vec![];
    keyboard_layouts.extend(keyboard_layouts_with_strategy);
    keyboard_layouts.extend(keyboard_layouts_with_depth);
    keyboard_layouts
}

fn apply_starting_point_to_keyboard_layout(keyboard_layout: Vec<Vec<String>>, starting_point: StartingPoint) -> Vec<Vec<Vec<String>>> {
    match starting_point {
        StartingPoint::TopLeft => { vec![keyboard_layout] }
        StartingPoint::BottomLeft => { vec![reverse_row_order(&keyboard_layout)] }
        StartingPoint::TopRight => { vec![reverse_column_order(&keyboard_layout)] }
        StartingPoint::BottomRight => {
            let reversed_rows = reverse_row_order(&keyboard_layout);
            vec![reverse_column_order(&reversed_rows)]
        }
        StartingPoint::All => { 
            vec![
                keyboard_layout.to_vec(),
                reverse_row_order(&keyboard_layout),
                reverse_column_order(&keyboard_layout),
                reverse_column_order(&reverse_row_order(&keyboard_layout))
            ]
        }
    }
}

fn apply_depths_to_keyboard_layouts(keyboard_layouts: &Vec<Vec<Vec<String>>>, min_depth: usize, max_depth: usize) -> Vec<Vec<Vec<String>>> {
    let mut keyboard_layouts_with_depth = vec![];
    for keyboard_layout in keyboard_layouts {
        let keyboard_layout_with_depth = create_keyboard_layout_with_depths(keyboard_layout, min_depth, max_depth);
        keyboard_layouts_with_depth.extend(keyboard_layout_with_depth);
    }
    keyboard_layouts_with_depth
}

fn apply_strategy_to_keyboard_layout(keyboard_layout: Vec<Vec<String>>, strategy: &Strategy) -> Vec<Vec<Vec<String>>> {
    match strategy {
        Strategy::Horizontal => {
            vec![keyboard_layout]
        }
        Strategy::Vertical => {
            let vertical_layout = turn_horizontal_into_vertical_keyboard_layout(&keyboard_layout);
            vec![vertical_layout]
        }
        Strategy::All => {
            let vertical_layout = turn_horizontal_into_vertical_keyboard_layout(&keyboard_layout);
            vec![keyboard_layout, vertical_layout]
        }
    }
}

fn create_keyboard_layout_with_depths(keyboard_layout: &Vec<Vec<String>>, from_depth: usize, to_depth: usize) -> Vec<Vec<Vec<String>>> {
    let mut keyboard_layouts = vec![];
    for depth in from_depth..to_depth {
        keyboard_layouts.push(create_layout_with_depth(keyboard_layout, depth+1));
    }
    keyboard_layouts
}

fn get_keyboard_layout_string_capacity(keyboard_layout: &Vec<Vec<String>>) -> usize {
    keyboard_layout
        .iter()
        .fold(0, |count, keyboard_layout_row| {
            count + keyboard_layout_row.len()
        })
}

fn merge_keyboard_layout_into_a_string(keyboard_layout: &Vec<Vec<String>>) -> String {
    let string_capacity = get_keyboard_layout_string_capacity(keyboard_layout);
    let initial_string = String::with_capacity(string_capacity);

    keyboard_layout.iter().fold(initial_string, |sum, row| {
        row.iter().fold(sum, |mut sum2, item| {
            sum2.push_str(item);
            sum2
        })
    })
}

fn turn_horizontal_into_vertical_keyboard_layout(
    keyboard_layout: &Vec<Vec<String>>,
) -> Vec<Vec<String>> {
    let mut new_layout: Vec<Vec<String>> = vec![];

    let string_length_max = keyboard_layout.iter().fold(0, |count, item| {
        let new_count = item.len();
        if count < new_count {
            new_count
        } else {
            count
        }
    });
    let vector_length = keyboard_layout.len();

    for i in 0..string_length_max {
        let mut new_row = vec![];
        for j in 0..vector_length {
            let row = &keyboard_layout[j];
            if i < row.len() {
                let item = &row[i];
                new_row.push(item.to_string());
            };
        }
        new_layout.push(new_row);
    }

    new_layout
}

fn create_layout_with_depth(layout: &Vec<Vec<String>>, depth: usize) -> Vec<Vec<String>> {
    
    let mut new_matrix: Vec<Vec<String>> = vec![];
    
    if depth <= 0 {
        return new_matrix
    }
    
    let row_length = layout.iter().fold(0, |count, item| {
        let new_count = item.len();
        if count < new_count {
            new_count
        } else {
            count
        }
    });
    
    let new_depth = if depth >= row_length {
        row_length-1
    } else {
        depth
    };
    
    for j in 0..row_length-new_depth {
        for i in 0..layout.len() {
            let new_row: Vec<String> = layout[i].iter()
                .skip(j)
                .take(1+new_depth)
                .map(|x| x.to_string())
                .collect();
            new_matrix.push(new_row);
        }
    }
    
    new_matrix
}

fn reverse_row_order(keyboard_layout: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    
    // What is the difference here?
    
    let mut keyboard_layout_rev = keyboard_layout.to_vec();
    keyboard_layout_rev.reverse();
    keyboard_layout_rev

    // keyboard_layout.iter()
    //     .rev()
    //     .map(|v| v.to_vec())
    //     .collect()
}

fn reverse_column_order(keyboard_layout: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    
    // What is the difference here?
    
    let mut keyboard_layout_rev = keyboard_layout.to_vec();
    for row in &mut keyboard_layout_rev {
        row.reverse();
    }
    keyboard_layout_rev

    // keyboard_layout.iter()
    //     .map(|v| v.iter()
    //         .rev()
    //         .map(|v| v.to_string())
    //         .collect()
    //     )
    //     .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_and_horizontal_strategy_creates_one_keyboard_each () {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 1);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 1);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::All, 0, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 2);
    }
    
    #[test]
    fn vertical_and_horizontal_strateg_with_depth_of_one_creates_two_keyboards_each () {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 1, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 2);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 1, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 2);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::All, 1, StartingPoint::TopLeft);
        assert!(created_keyboard_layout.len() == 4);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_and_depth_one() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 1, StartingPoint::TopLeft);

        assert_eq!("12qwas23wesd34erdf".to_string(), created_keyboard_layout[1]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_and_depth_one() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 1, StartingPoint::TopLeft);

        assert_eq!("1q2w3e4rqawsedrf".to_string(), created_keyboard_layout[1]);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_1_row() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),]
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0, StartingPoint::TopLeft);

        assert_eq!("1234".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_4_rows() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0, StartingPoint::TopLeft);

        assert_eq!(
            "123456qwertyasdfghzxcvbn".to_string(),
            created_keyboard_layout[0]
        );
    }

    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_jagged_rows() {
        let keyboard_layout = vec![
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0, StartingPoint::TopLeft);

        assert_eq!("qwertyasdzxcvbn".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_1_row() {
        let keyboard_layout = vec![vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),]];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0, StartingPoint::TopLeft);

        assert_eq!("1234".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_4_rows() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0, StartingPoint::TopLeft);

        assert_eq!(
            "1qaz2wsx3edc4rfv5tgb6yhn".to_string(),
            created_keyboard_layout[0]
        );
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_jagged_rows() {
        let keyboard_layout = vec![
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(), "s".to_string(), "d".to_string()],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0, StartingPoint::TopLeft);

        assert_eq!("qazwsxedcrvtbyn".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn turn_keyboard_layout_vertical() {
        let keyboard_layout = vec![
            vec!["q".to_string(), "w".to_string(), "e".to_string()],
            vec!["a".to_string(), "s".to_string(), "d".to_string()],
            vec!["z".to_string(), "x".to_string(), "c".to_string()],
        ];

        let new_layout = turn_horizontal_into_vertical_keyboard_layout(&keyboard_layout);

        assert_eq!(
            vec![
                vec!["q".to_string(), "a".to_string(), "z".to_string()],
                vec!["w".to_string(), "s".to_string(), "x".to_string()],
                vec!["e".to_string(), "d".to_string(), "c".to_string()],
            ],
            new_layout
        );
    }

    #[test]
    fn turn_keyboard_layout_into_string() {
        let keyboard_layout = vec![
            vec!["q".to_string(), "w".to_string(), "e".to_string()],
            vec!["a".to_string(), "s".to_string(), "d".to_string()],
            vec!["z".to_string(), "x".to_string(), "c".to_string()],
        ];

        let new_layout = merge_keyboard_layout_into_a_string(&keyboard_layout);

        assert_eq!("qweasdzxc".to_string(), new_layout);
    }
    
    #[test]
    fn create_new_layout_with_a_depth_of_one() {
        let depth = 1;
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let created_keyboard_layout = create_layout_with_depth(&keyboard_layout, depth);
        
        let expected_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),],
            vec!["q".to_string(),"w".to_string(),],
            vec!["a".to_string(),"s".to_string(),],
            vec!["2".to_string(),"3".to_string(),],
            vec!["w".to_string(),"e".to_string(),],
            vec!["s".to_string(),"d".to_string(),],
        ];
        
        assert_eq!(expected_keyboard_layout, created_keyboard_layout);
    }
    
    #[test]
    fn create_new_layout_with_a_depth_of_zero() {
        
        let depth = 0;
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let created_keyboard_layout = create_layout_with_depth(&keyboard_layout, depth);
        
        let expected_keyboard_layout: Vec<Vec<String>> = vec![];
        
        assert_eq!(expected_keyboard_layout, created_keyboard_layout);
    }
    
    #[test]
    fn create_new_layout_with_a_depth_of_two() {
        let depth = 2;
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let created_keyboard_layout = create_layout_with_depth(&keyboard_layout, depth);
        
        let expected_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        assert_eq!(expected_keyboard_layout, created_keyboard_layout);
    }
    
    #[test]
    fn create_new_layout_with_a_depth_of_three() {
        let depth = 3;
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let created_keyboard_layout = create_layout_with_depth(&keyboard_layout, depth);
        
        let expected_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        assert_eq!(expected_keyboard_layout, created_keyboard_layout);
    }
    
    #[test]
    fn create_new_layout_with_a_depth_of_four() {
        let depth = 4;
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let created_keyboard_layout = create_layout_with_depth(&keyboard_layout, depth);
        
        let expected_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        assert_eq!(expected_keyboard_layout, created_keyboard_layout);
    }
    
    #[test]
    fn horizontal_keyboard_layout_bottom_to_top_left_to_right() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let reversed_rows_horizontal_keyboard_layout = reverse_row_order(&horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
        ];
        
        assert_eq!(expected_keyboard_layout, reversed_rows_horizontal_keyboard_layout);
    }
    
    #[test]
    fn vertical_keyboard_layout_bottom_to_top_left_to_right() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        let reversed_rows_horizontal_keyboard_layout = reverse_row_order(&horizontal_keyboard_layout);
        let reversed_rows_vertical_keyboard_layout = turn_horizontal_into_vertical_keyboard_layout(&reversed_rows_horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["a".to_string(), "q".to_string(), "1".to_string()],
            vec!["s".to_string(), "w".to_string(), "2".to_string()],
            vec!["d".to_string(), "e".to_string(), "3".to_string()],
        ];
            
        assert_eq!(expected_keyboard_layout, reversed_rows_vertical_keyboard_layout);
    }
    
    #[test]
    fn horizontal_keyboard_layout_top_to_bottom_right_to_left() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let reversed_columns_horizontal_keyboard_layout = reverse_column_order(&horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["3".to_string(),"2".to_string(),"1".to_string()],
            vec!["e".to_string(),"w".to_string(),"q".to_string()],
            vec!["d".to_string(),"s".to_string(),"a".to_string()],
        ];
        
        assert_eq!(expected_keyboard_layout, reversed_columns_horizontal_keyboard_layout);
    }
    
    #[test]
    fn vertical_keyboard_layout_top_to_bottom_right_to_left() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        let reversed_columns_horizontal_keyboard_layout = reverse_column_order(&horizontal_keyboard_layout);
        let reversed_columns_vertical_keyboard_layout = turn_horizontal_into_vertical_keyboard_layout(&reversed_columns_horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["3".to_string(), "e".to_string(), "d".to_string()],
            vec!["2".to_string(), "w".to_string(), "s".to_string()],
            vec!["1".to_string(), "q".to_string(), "a".to_string()],
        ];
        
        assert_eq!(expected_keyboard_layout, reversed_columns_vertical_keyboard_layout);
    }
    
    #[test]
    fn horizontal_keyboard_layout_bottom_to_top_right_to_left() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];

        let reversed_rows_horizontal_keyboard_layout = reverse_row_order(&horizontal_keyboard_layout);
        let reversed_columns_horizontal_keyboard_layout = reverse_column_order(&reversed_rows_horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["d".to_string(),"s".to_string(),"a".to_string()],
            vec!["e".to_string(),"w".to_string(),"q".to_string()],
            vec!["3".to_string(),"2".to_string(),"1".to_string()],
        ];
        
        assert_eq!(expected_keyboard_layout, reversed_columns_horizontal_keyboard_layout);
    }
    
     #[test]
    fn vertical_keyboard_layout_bottom_to_top_right_to_left() {
        
        let horizontal_keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),],
        ];
        
        let reversed_rows_horizontal_keyboard_layout = reverse_row_order(&horizontal_keyboard_layout);
        let reversed_columns_horizontal_keyboard_layout = reverse_column_order(&reversed_rows_horizontal_keyboard_layout);
        let reversed_columns_vertical_keyboard_layout = turn_horizontal_into_vertical_keyboard_layout(&reversed_columns_horizontal_keyboard_layout);
        
        let expected_keyboard_layout = vec![
            vec!["d".to_string(), "e".to_string(), "3".to_string()],
            vec!["s".to_string(), "w".to_string(), "2".to_string()],
            vec!["a".to_string(), "q".to_string(), "1".to_string()],
        ];
        
        assert_eq!(expected_keyboard_layout, reversed_columns_vertical_keyboard_layout);
    }
}
