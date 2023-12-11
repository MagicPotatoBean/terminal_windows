use std::char;
#[derive(Clone)]
pub struct Window {
    text: Vec<String>,
    pub windows: Vec<Window>,
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
}
impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut print_text = String::default();
        for line in &self.text {
            print_text.push_str(line);
            print_text.push('\n');
        }
        write!(f, "{}", print_text)
    }
}
impl Window {
    pub fn draw(
        &mut self,
        style: Option<BoxStyle>,
    ) -> Result<(), DrawBoxError> {
        let w = self.width;
        let h = self.height;
        let x = self.x;
        let y = self.y;

        let mut new_state = self.clone();
        let style = match style {
            Some(style) => style,
            None => BoxStyle::default(),
        };
        {
            if w < 1 || h < 1 || x + w > new_state.width || y + h > new_state.height {
                println!("W or H out of bounds.");
                return Err(DrawBoxError::BoxOutOfBounds);
            }
            let line = match new_state.text.get_mut(y) {
                Some(ok_val) => ok_val,
                None => {
                    println!("Out of bounds");
                    return Err(DrawBoxError::BoxOutOfBounds);
                }
            };
            println!(
                "{:?}",
                index_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?
                    ..=index_from_char(line, x + w).ok_or(DrawBoxError::BoxOutOfBounds)?
            );
            line.replace_range(
                index_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?
                    ..=index_from_char(line, x + w).ok_or(DrawBoxError::BoxOutOfBounds)?,
                &style.horizontal.to_string().as_str().repeat(w + 1),
            );
            let line = match new_state.text.get_mut(y + h) {
                Some(ok_val) => ok_val,
                None => {
                    println!("1) Out of bounds");
                    return Err(DrawBoxError::BoxOutOfBounds);
                }
            };
            if line.len() < x + w {
                return Err(DrawBoxError::BoxOutOfBounds);
            }
            line.replace_range(
                index_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?
                    ..=index_from_char(line, x + w).ok_or(DrawBoxError::BoxOutOfBounds)?,
                &style.horizontal.to_string().as_str().repeat(w + 1),
            );
        }
        println!("Horizontals done");
        for line_num in (y + 1)..(y + h) {
            replace_letter_2d(&mut new_state.text, x, line_num, style.vertical)?;
            replace_letter_2d(&mut new_state.text, x + w, line_num, style.vertical)?;
        }
        replace_letter_2d(&mut new_state.text, x, y, style.tl)?;
        replace_letter_2d(&mut new_state.text, x + w, y, style.tr)?;
        replace_letter_2d(&mut new_state.text, x, y + h, style.bl)?;
        replace_letter_2d(&mut new_state.text, x + w, y + h, style.br)?;
        *self = new_state;
        Ok(())
    }
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        let mut new_vec: Vec<String> = Vec::new();
        for _ in 0..(height - 1) {
            new_vec.push(" ".repeat(width))
        }
        Window { text: new_vec, width, height, windows: Vec::new(), x, y }
    }
    // pub fn from_vec(data: Vec<String>) -> Option<Self> {
    //     let mut line_len = None;
    //     let len = data.len();
    //     for line in data.iter() {
    //         if let Some(last_val) = line_len {
    //             if line.chars().count() != last_val {
    //                 return None
    //             }
    //         }
    //         line_len = Some(line.chars().count());
    //     }
    //     Some(Window { text: data, width: line_len?, height: len, windows: Vec::new(), x, y })
    // }
}
#[derive(Debug)]
pub enum DrawBoxError {
    BoxOutOfBounds,
}
fn index_from_char(text: &str, char_number: usize) -> Option<usize> {
    text.char_indices().nth(char_number).map(|(pos, _)| pos)
}
fn range_from_char(text: &str, char_number: usize) -> Option<std::ops::Range<usize>> {
    text.char_indices()
        .nth(char_number)
        .map(|(pos, chr)| pos..(pos + chr.len_utf8()))
}
pub struct BoxStyle {
    horizontal: char,
    vertical: char,
    tl: char,
    tr: char,
    br: char,
    bl: char,
}
impl Default for BoxStyle {
    fn default() -> Self {
        Self {
            horizontal: '─',
            vertical: '│',
            tl: '╭',
            tr: '╮',
            br: '╯',
            bl: '╰',
        }
    }
}
fn replace_letter_2d(
    text: &mut [String],
    x: usize,
    y: usize,
    new_char: char,
) -> Result<(), DrawBoxError> {
    let line = match text.get_mut(y) {
        Some(ok_val) => ok_val,
        None => {
            return Err(DrawBoxError::BoxOutOfBounds);
        }
    };
    line.replace_range(
        range_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?,
        new_char.to_string().as_str(),
    );
    println!("Replaced ({}, {})", x, y);
    Ok(())
}
