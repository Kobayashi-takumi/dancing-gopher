use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct App {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub counter: i64,
}

impl App {
    const NUM: [f64; 2] = [10.0, 20.0];
    const X: [f64; 2] = [1.0, -1.0];
    const VOCABULARY: [&str; 3] = ["> I'm Gopher!", "> Hi!", "> I love Golang!"];
    pub fn new(x: f64, y: f64) -> App {
        App {
            x,
            y,
            text: Self::VOCABULARY[0].to_string(),
            counter: 0,
        }
    }
    pub fn right(&mut self) {
        self.x += 3.0;
    }
    pub fn left(&mut self) {
        self.x -= 3.0;
    }
    pub fn jump(&mut self) {
        if self.y >= 0_f64 {
            let y = *Self::NUM.choose(&mut rand::thread_rng()).unwrap();
            if y != 0.0 {
                self.y = y
            }
        }
    }
    fn fall(&mut self) {
        if self.y != 0_f64 {
            self.y = 0.0;
        }
    }
    fn dash(&mut self) {
        self.x -= *Self::X.choose(&mut rand::thread_rng()).unwrap();
    }
    pub fn moving(&mut self) {
        self.dash();
        self.fall();
        self.speak();
    }
    pub fn speak(&mut self) {
        if self.counter >= 10 {
            let text = *Self::VOCABULARY.choose(&mut rand::thread_rng()).unwrap();
            self.text = text.to_string();
            self.counter = 0;
        }
        self.counter += 1;
    }
}
