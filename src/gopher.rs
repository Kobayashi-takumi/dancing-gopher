use ratatui::style::Color;

const CREAM: Color = Color::Rgb(255, 249, 197);
const BLUE: Color = Color::Rgb(170, 231, 254);
const WIHTE: Color = Color::White;
const BLACK: Color = Color::Black;

pub fn gopher() -> Vec<Vec<(f64, f64, f64, Color)>> {
    vec![
        vec![(6.0, 1.0, 0.0, BLUE), (17.0, 1.0, 0.0, BLUE)],
        vec![(5.0, 2.0, -1.0, BLUE), (16.0, 2.0, -1.0, BLUE)],
        vec![(5.0, 14.0, -2.0, BLUE)],
        vec![(4.0, 16.0, -3.0, BLUE)],
        vec![(4.0, 16.0, -4.0, BLUE)],
        vec![
            (4.0, 16.0, -5.0, BLUE),
            (6.0, 2.0, -5.0, WIHTE),
            (16.0, 2.0, -5.0, WIHTE),
        ],
        vec![
            (4.0, 16.0, -6.0, BLUE),
            (6.0, 2.0, -6.0, WIHTE),
            (7.0, 1.0, -6.0, BLACK),
            (16.0, 2.0, -6.0, WIHTE),
            (17.0, 1.0, -6.0, BLACK),
        ],
        vec![
            (4.0, 16.0, -7.0, BLUE),
            (6.0, 2.0, -7.0, WIHTE),
            (16.0, 2.0, -7.0, WIHTE),
        ],
        vec![
            (4.0, 16.0, -8.0, BLUE),
            (6.0, 2.0, -8.0, WIHTE),
            (16.0, 2.0, -8.0, WIHTE),
        ],
        vec![(4.0, 16.0, -9.0, BLUE)],
        vec![
            (4.0, 16.0, -10.0, BLUE),
            (10.0, 5.0, -10.0, CREAM),
            (12.0, 2.0, -10.0, BLACK),
            (2.0, 2.0, -9.0, CREAM),
            (20.0, 2.0, -9.0, CREAM),
        ],
        vec![(4.0, 16.0, -11.0, BLUE), (10.0, 4.0, -11.0, CREAM)],
        vec![(4.0, 16.0, -12.0, BLUE), (10.0, 4.0, -12.0, CREAM)],
        vec![(4.0, 16.0, -13.0, BLUE), (11.0, 1.0, -13.0, WIHTE)],
        vec![(4.0, 16.0, -14.0, BLUE)],
        vec![(4.0, 16.0, -15.0, BLUE)],
        vec![(4.0, 16.0, -16.0, BLUE)],
        vec![(4.0, 16.0, -17.0, BLUE)],
        vec![(5.0, 14.0, -18.0, BLUE)],
        vec![(6.0, 12.0, -19.0, BLUE)],
        vec![(6.0, 2.0, -20.0, CREAM), (18.0, 2.0, -20.0, CREAM)],
    ]
}
