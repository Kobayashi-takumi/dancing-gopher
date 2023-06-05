use ratatui::style::Color;

const CREAM: Color = Color::Rgb(255, 249, 197);
const BLUE: Color = Color::Rgb(170, 231, 254);
const WIHTE: Color = Color::White;
const BLACK: Color = Color::Black;

pub fn gopher() -> Vec<Vec<(f64, f64, f64, Color)>> {
    vec![
        vec![(7.0, 1.0, 0.0, BLUE), (17.0, 1.0, 0.0, BLUE)],
        vec![(6.0, 2.0, -1.0, BLUE), (16.0, 2.0, -1.0, BLUE)],
        vec![(6.0, 12.0, -2.0, BLUE)],
        vec![(5.0, 14.0, -3.0, BLUE)],
        vec![(5.0, 14.0, -4.0, BLUE)],
        vec![
            (5.0, 14.0, -5.0, BLUE),
            // 目
            (6.0, 3.0, -5.0, WIHTE),
            (15.0, 3.0, -5.0, WIHTE),
        ],
        vec![
            (5.0, 14.0, -6.0, BLUE),
            // 目
            (6.0, 3.0, -6.0, WIHTE),
            (15.0, 3.0, -6.0, WIHTE),
            (7.0, 1.0, -6.0, BLACK),
            (16.0, 1.0, -6.0, BLACK),
        ],
        vec![
            (5.0, 14.0, -7.0, BLUE),
            // 目
            (6.0, 3.0, -7.0, WIHTE),
            (15.0, 3.0, -7.0, WIHTE),
        ],
        vec![(5.0, 14.0, -8.0, BLUE)],
        vec![
            (5.0, 14.0, -9.0, BLUE),
            // 鼻
            (9.0, 5.0, -9.0, CREAM),
        ],
        vec![
            (5.0, 14.0, -10.0, BLUE),
            // 手
            (3.0, 2.0, -10.0, CREAM),
            (19.0, 2.0, -10.0, CREAM),
            // 鼻
            (10.0, 3.0, -10.0, CREAM),
        ],
        vec![
            (5.0, 14.0, -11.0, BLUE),
            // 鼻
            (11.0, 1.0, -11.0, WIHTE),
        ],
        vec![(5.0, 14.0, -12.0, BLUE)],
        vec![(5.0, 14.0, -13.0, BLUE)],
        vec![(5.0, 14.0, -14.0, BLUE)],
        vec![(5.0, 14.0, -15.0, BLUE)],
        vec![(5.0, 14.0, -16.0, BLUE)],
        vec![(5.0, 14.0, -17.0, BLUE)],
        vec![(6.0, 12.0, -18.0, BLUE)],
        vec![(6.0, 1.0, -19.0, CREAM), (17.0, 1.0, -19.0, CREAM)],
        vec![(6.0, 1.0, -20.0, CREAM), (17.0, 1.0, -20.0, CREAM)],
    ]
}
