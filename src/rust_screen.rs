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
        for line in self.draw(None).unwrap() {
            print_text.push_str(&line);
            print_text.push('\n');
        }
        write!(f, "{}", print_text)
    }
}
impl Window {
    pub fn draw(
        &self,
        style: Option<BoxStyle>,
    ) -> Result<Vec<String>, DrawBoxError> {
        for data in self.content.iter() {
            match data {
                WindowContent::SubWindow(window) => {
                    window.draw(style);
                    todo!()
                },
                WindowContent::Text(text) => {
                    todo!()
                },
            };
        }

        let w = self.width;
        let h = self.height;
        let x = self.x;
        let y = self.y;

        let mut new_state = self.clone();
        let style = match style {
            Some(style) => style,
            None => BoxStyle::default(),
        };
        if let Some(title) = &self.name {
            new_state.text = title_bar(&mut new_state.text, x..(x + w - 1), y, Some(title.to_owned()) , style)
        } else {
            new_state.text = horizontal_line(&mut new_state.text, x..(x + w - 1), y, style)?;
        }
        new_state.text = horizontal_line(&mut new_state.text, x..(x + w - 1), y + h, style)?;
        new_state.text = vertical_line(&mut new_state.text, x, y..(y+h - 1), style)?;
        new_state.text = vertical_line(&mut new_state.text, x + w, (y)..(y+h - 1), style)?;
        // Placing corners
        replace_letter_2d(&mut new_state.text, x, y, style.tl)?;
        replace_letter_2d(&mut new_state.text, x + w, y, style.tr)?;
        replace_letter_2d(&mut new_state.text, x, y + h, style.bl)?;
        replace_letter_2d(&mut new_state.text, x + w, y + h, style.br)?;
        Ok(new_state.text)
    }
    pub fn new(x: usize, y: usize, width: usize, height: usize, name: Option<String>) -> Self {
        let mut new_vec: Vec<String> = Vec::new();
        for _ in 0..(height + 2) {
            new_vec.push(" ".repeat(width + 2))
        }
        Window { text: new_vec, width:width + 1, height:height + 1, x, y, name, content: Vec::new()}
    }
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
    Ok(())
}
fn title_bar(data: &mut Vec<String>, x: Range<usize>, y: usize, title: Option<String>, style: BoxStyle) -> Vec<String> {
    let mut new_title: String = String::default();
    let width = x.end - x.start - 2;
    if let Some(title) = title {
        if title.chars().count() + 3 < width {
            let trailing_horizontals = width - title.chars().count() + 1;
            new_title.push(style.tl);
            new_title.push(style.horizontal);
            new_title.push_str(&title);
            new_title.push_str(style.horizontal.to_string().repeat(trailing_horizontals).as_str());
            new_title.push(style.tr);
        } else if width < 2 {
            new_title.push(style.tl);
            new_title.push_str(style.horizontal.to_string().repeat(width).as_str());
            new_title.push(style.tr);
        } else {
            new_title.push(style.tl);
            new_title.push(style.horizontal);
            let split_point = index_from_char(&title, width - 1).unwrap();
            new_title.push_str(title.split_at(split_point).0);
            new_title.push_str("..");
            new_title.push(style.tr);
        }
    } else {
        new_title.push(style.tl);
        new_title.push_str(style.horizontal.to_string().repeat(width).as_str());
        new_title.push(style.tr);
    }
    let mut new_data: Vec<String> = data.clone();
    for char_num in (x.start + 1)..=(x.end) {
        replace_letter_2d(&mut new_data, char_num, y, new_title.char_indices().nth(char_num).unwrap().1).unwrap();
    };
    new_data
}
fn vertical_line(data: &mut Vec<String>, x: usize, y: Range<usize>, style: BoxStyle) -> Result<Vec<String>, DrawBoxError> {
    let mut new_data: Vec<String> = data.clone();
    for line_num in (y.start + 1)..=(y.end) {
        replace_letter_2d(&mut new_data, x, line_num, style.vertical)?;
    };
    Ok(new_data)
}
fn horizontal_line(data: &mut Vec<String>, x: Range<usize>, y: usize, style: BoxStyle) -> Result<Vec<String>, DrawBoxError> {
    let mut new_data: Vec<String> = data.clone();
    for char_num in (x.start + 1)..=(x.end) {
        replace_letter_2d(&mut new_data, char_num, y, style.horizontal)?;
    };
    Ok(new_data)
}