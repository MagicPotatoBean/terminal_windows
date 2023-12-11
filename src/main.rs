mod rust_screen;
use rust_screen::*;
fn main() {
    let mut state = Window::new(0, 0, 10, 5, Some("Test".to_owned()));
    state.content.push(WindowContent::SubWindow(Window::new(1, 1, 3, 3, None)));
    // let mut state = Window::from_vec(state_vec).expect("testing");   
    println!("{:#?}", state.draw(None));
    print!("{}", state);
    println!("---")
}