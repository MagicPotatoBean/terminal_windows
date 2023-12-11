mod rust_screen;
use rust_screen::ScreenState;
fn main() {
    let mut state_vec: Vec<String> = Vec::new();
    state_vec.push("╭──────╮".into());
    state_vec.push("│      │".into());
    state_vec.push("│      │".into());
    state_vec.push("│      │".into());
    state_vec.push("│      │".into());
    state_vec.push("│      │".into());
    state_vec.push("│      │".into());
    state_vec.push("╰──────╯".into());
    let mut state = ScreenState::new();
    state.set_text(state_vec);
    let _ = state.draw_box(2, 2, 4, 5);
    print!("{}", state);
}