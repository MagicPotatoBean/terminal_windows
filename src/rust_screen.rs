use std::{char, ops::Range, iter::StepBy};
#[derive(Clone)]
pub struct Window {
    text: Vec<String>,
    pub width: usize,
    pub height: usize,
    pub x: usize,
    pub y: usize,
    pub name: Option<String>,
    pub content: Vec<WindowContent>,
}
#[derive(Clone)]
pub enum WindowContent {
    SubWindow(Window),
    Text(String),
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
            // if w < 1 || h < 1 || x + w > new_state.width || y + h > new_state.height {
            //     println!("W or H out of bounds.");
            //     return Err(DrawBoxError::BoxOutOfBounds);
            // }
            let line = match new_state.text.get_mut(y) {
                Some(ok_val) => ok_val,
                None => {
                    println!("Out of bounds");
                    todo!()
                    // return Err(DrawBoxError::BoxOutOfBounds);
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
                &title_bar(w, self.name.clone(), style),
            );
            let line = match new_state.text.get_mut(y + h) {
                Some(ok_val) => ok_val,
                None => {
                    println!("1) Out of bounds");
                    todo!()
                    // return Err(DrawBoxError::BoxOutOfBounds);
                }
            };
            if line.len() < x + w {
                // return Err(DrawBoxError::BoxOutOfBounds);
            }
            line.replace_range(
                index_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?
                    ..=index_from_char(line, x + w).ok_or(DrawBoxError::BoxOutOfBounds)?,
                &style.horizontal.to_string().as_str().repeat(w + 1),
            );
        }
        println!("Horizontals done");
        for line_num in (y + 1)..=(y + h) {
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
    pub fn new(x: usize, y: usize, width: usize, height: usize, name: Option<String>) -> Self {
        let mut new_vec: Vec<String> = Vec::new();
        for _ in 0..(height + 2) {
            new_vec.push(" ".repeat(width + 2))
        }
        Window { text: new_vec, width:width + 1, height:height + 1, x, y, name, content: Vec::new()}
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
#[derive(Clone, Copy)]
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
            todo!()
            // return Err(DrawBoxError::BoxOutOfBounds);
        }
    };
    line.replace_range(
        range_from_char(line, x).ok_or(DrawBoxError::BoxOutOfBounds)?,
        new_char.to_string().as_str(),
    );
    println!("Replaced ({}, {})", x, y);
    Ok(())
}
fn title_bar(width: usize, title: Option<String>, style: BoxStyle) -> String {
    if let Some(title) = title {
        if title.chars().count() + 3 < width {
            let trailing_horizontals = width - title.chars().count() - 1;
            let mut new_title = String::default();
            new_title.push(style.tl);
            new_title.push(style.horizontal);
            new_title.push_str(&title);
            new_title.push_str(style.horizontal.to_string().repeat(trailing_horizontals).as_str());
            return new_title;
        } else {
            let trailing_horizontals = width - title.chars().count() - 1;
            let mut new_title = String::default();
            new_title.push(style.tl);
            new_title.push(style.horizontal);
            let split_point = index_from_char(&title, width - 4).unwrap();
            new_title.push_str(&title.split_at(split_point).0);
            new_title.push_str("...");
            new_title.push_str(style.horizontal.to_string().repeat(trailing_horizontals).as_str());
            new_title.push(style.tr);
            return new_title;
        }
    } else {
        let mut new_title = String::default();
        new_title.push(style.tl);
        new_title.push_str(style.horizontal.to_string().repeat(width).as_str());
        new_title.push(style.tr);
        return new_title;
    }
}
fn vertical_line(data: &mut Vec<String>, x: usize, y: Range<usize>, style: BoxStyle) -> Result<Vec<String>, DrawBoxError> {
    let mut new_data: Vec<String> = data.clone();
    for line_num in (y.start + 1)..=(y.end) {
        replace_letter_2d(&mut new_data, x, line_num, style.vertical)?;
    };
    Ok(new_data)
}
fn horizontal_line(data: &mut Vec<String>, x: Range<usize>, y: usize, style: BoxStyle) -> Result<Vec<String>, DrawBoxError> {
    let mut new_data = data.clone();
    let line = match new_data.get_mut(y) {
        Some(ok_val) => ok_val,
        None => {
            println!("Out of bounds");
            return Err(DrawBoxError::BoxOutOfBounds);
        }
    };
    println!(
        "{:?}",
        index_from_char(line, x.start).ok_or(DrawBoxError::BoxOutOfBounds)?
            ..=index_from_char(line, x.end).ok_or(DrawBoxError::BoxOutOfBounds)?
    );
    line.replace_range(
        index_from_char(line, x.start).ok_or(DrawBoxError::BoxOutOfBounds)?
            ..=index_from_char(line, x.end).ok_or(DrawBoxError::BoxOutOfBounds)?,
            style.horizontal.to_string().as_str(),
    );
    Ok(new_data)
}