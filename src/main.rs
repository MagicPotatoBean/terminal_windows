mod rust_screen;
use rust_screen::Window;
fn main() {
    let mut state_vec: Vec<String> = Vec::new();
    state_vec.push("╭─Test window──────╮".into());
    state_vec.push("│                  │".into());
    state_vec.push("│                  │".into());
    state_vec.push("│                  │".into());
    state_vec.push("│                  │".into());
    state_vec.push("│                  │".into());
    state_vec.push("│                  │".into());
    state_vec.push("╰──────────────────╯".into());
    let mut state = Window::new(0, 0, 5, 5);
    // let mut state = Window::from_vec(state_vec).expect("testing");   
    println!("{:#?}", state.draw(None));
    print!("{}", state);
}