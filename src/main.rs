use crossterm::{terminal::disable_raw_mode, terminal::enable_raw_mode};
fn main() {
    enable_raw_mode().unwrap();
    println!("Hello, world!");
    disable_raw_mode().unwrap();
}
