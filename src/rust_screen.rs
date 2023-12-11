pub struct ScreenState {
    text: Vec<String>,
}
impl std::fmt::Display for ScreenState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_text = String::default();
        for line in &self.text {
            print_text.push_str(line);
            print_text.push('\n');
        }
        write!(f, "{}", print_text)
    }
}
impl ScreenState {
    pub fn draw_box(&mut self, x: usize, y: usize, w: usize, h: usize) -> Result<(), DrawBoxError> {
            if w < 2 || h < 2 {
                return Err(DrawBoxError::BoxOutOfBounds);
            }
                let line = match self.text.get_mut(y) {
                    Some(ok_val) => {
                        if !ok_val.is_char_boundary(x) || !ok_val.is_char_boundary(x + w) {
                            return Err(DrawBoxError::BoxOutOfBounds)
                        }
                        ok_val
                    },
                    None => {
                        println!("Out of bounds");
                        return Err(DrawBoxError::BoxOutOfBounds)
                    },
                };
                line.replace_range(x..=(x + w), &"#".repeat(w));
                let line = match self.text.get_mut(y + h) {
                    Some(ok_val) => {
                        if !ok_val.is_char_boundary(x) || !ok_val.is_char_boundary(x + w) {
                            return Err(DrawBoxError::BoxOutOfBounds)
                        }
                        ok_val
                    },
                    None => {
                        println!("Out of bounds");
                        return Err(DrawBoxError::BoxOutOfBounds)
                    },
                };
                if line.len() < x + w {
                    return Err(DrawBoxError::BoxOutOfBounds)
                }
                line.replace_range(x..=(x + w), &"#".repeat(w));
        todo!()
    }
    pub fn new() -> Self {
        ScreenState { text: Vec::new() }
    }
    pub fn set_text(&mut self, text: Vec<String>) {
        self.text = text
    }
}
pub enum DrawBoxError {
    BoxOutOfBounds,
}
