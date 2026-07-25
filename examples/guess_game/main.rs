// Declare the module so Rust compiles the sibling file
mod guess_game;

fn main() {
    println!("Number Guessing Game!");
    // Call the function using the module namespace
    guess_game::guess_number();
}
