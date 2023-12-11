mod rust_screen;
use rust_screen::*;
fn main() {
    let mut state = Window::new(0, 0, 20, 10, Some("Desktop".to_owned()));
    // state.content.push(WindowContent::SubWindow(Window::new(1, 1, 3, 3, None)));
    match state.draw(None) {
        Ok(_) => {},
        Err(err_val) => println!("{err_val:?}"),
    };
    println!("---");
    println!("{}", state);
    println!("---")
}