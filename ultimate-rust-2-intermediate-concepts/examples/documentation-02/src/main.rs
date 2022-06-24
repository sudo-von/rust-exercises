/**
    Outer documentation
    Number of pieces in the puzzle
    # History
    This is a separate paragraph.
    - Clickable link: [`PUZZLE_PIECES`]
    - [Another clickable link](`PUZZLE_PIECES`)
    - We tried `7`, but this is better.
**/
pub const PUZZLE_PIECES: u32 = 42;

/// This is a Puzzle!
pub struct Puzzle {
    /// Number of pieces
    pub num_pieces: u32,
    /// Descriptive name
    pub name: String,
}

fn main() {
    //! Hi! I'm your friendly Rust Puzzle
    //! Library documentation. Please
    //! come in, sit down, and have a cup
    //! of hot chocolate!
    println!("Hello, world!");
}
