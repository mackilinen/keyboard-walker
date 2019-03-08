#[derive(EnumString)]
pub enum Strategy {
    Horizontal,
    Vertical,
    All,
}

pub fn create_keyboard_layout(
    keyboard_layout: Vec<Vec<String>>,
    strategy: Strategy,
    depths: usize
) -> Vec<String> {
    match strategy {
        Strategy::Horizontal => {
            let mut keyboard_layouts = vec![];
            keyboard_layouts.push(merge_keyboard_layout_into_a_string(&keyboard_layout));
            
            for depth in 0..depths {
                let layout_with_depth = create_layout_with_depth(&keyboard_layout, depth+1);
                keyboard_layouts.push(merge_keyboard_layout_into_a_string(&layout_with_depth));
            }
            
            keyboard_layouts
        }
        Strategy::Vertical => {
            let mut keyboard_layouts = vec![];
            let vertical_layout = turn_horizontal_into_vertical_keyboard_layout(&keyboard_layout);
            keyboard_layouts.push(merge_keyboard_layout_into_a_string(&vertical_layout));
            
            for depth in 0..depths {
                let layout_with_depth = create_layout_with_depth(&vertical_layout, depth+1);
                keyboard_layouts.push(merge_keyboard_layout_into_a_string(&layout_with_depth));
            }
            
            keyboard_layouts
        }
        Strategy::All => {
            let mut keyboard_layouts = vec![];
            let vertical_layout = turn_horizontal_into_vertical_keyboard_layout(&keyboard_layout);
            
            keyboard_layouts.push(merge_keyboard_layout_into_a_string(&keyboard_layout));
            keyboard_layouts.push(merge_keyboard_layout_into_a_string(&vertical_layout));
            
            for depth in 0..depths {
                let mut layout_with_depth = create_layout_with_depth(&keyboard_layout, depth+1);
                keyboard_layouts.push(merge_keyboard_layout_into_a_string(&layout_with_depth));
                
                layout_with_depth = create_layout_with_depth(&vertical_layout, depth+1);
                keyboard_layouts.push(merge_keyboard_layout_into_a_string(&layout_with_depth));
            }
            
            keyboard_layouts
        }
    }
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
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0);
        assert!(created_keyboard_layout.len() == 1);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0);
        assert!(created_keyboard_layout.len() == 1);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::All, 0);
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
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 1);
        assert!(created_keyboard_layout.len() == 2);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 1);
        assert!(created_keyboard_layout.len() == 2);
        
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),"5".to_string(),"6".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),"t".to_string(),"y".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),"g".to_string(),"h".to_string(),],
            vec!["z".to_string(),"x".to_string(),"c".to_string(),"v".to_string(),"b".to_string(),"n".to_string(),],
        ];
        
        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::All, 1);
        assert!(created_keyboard_layout.len() == 4);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_and_depth_one() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 1);

        assert_eq!("12qwas23wesd34erdf".to_string(), created_keyboard_layout[1]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_and_depth_one() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),],
            vec!["q".to_string(),"w".to_string(),"e".to_string(),"r".to_string(),],
            vec!["a".to_string(),"s".to_string(),"d".to_string(),"f".to_string(),],
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 1);

        assert_eq!("1q2w3e4rqawsedrf".to_string(), created_keyboard_layout[1]);
    }
    
    #[test]
    fn create_a_new_keyboard_layout_with_horizontal_strategy_should_handle_1_row() {
        let keyboard_layout = vec![
            vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),]
        ];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0);

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

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0);

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

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Horizontal, 0);

        assert_eq!("qwertyasdzxcvbn".to_string(), created_keyboard_layout[0]);
    }

    #[test]
    fn create_a_new_keyboard_layout_with_vertical_strategy_should_handle_1_row() {
        let keyboard_layout = vec![vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string(),]];

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0);

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

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0);

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

        let created_keyboard_layout = create_keyboard_layout(keyboard_layout, Strategy::Vertical, 0);

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
}
