// #![allow(dead_code)]
use std::io::{self, Write};

use board;

pub fn reset_cursor_to_start() {
    // Moves to the cursor to the very top left (1st row and 1st col)
    // Good default and standard location for cursor so we know to
    // always count the rows and cols from the top left
    print!("\x1B[1;1H");
}

pub fn inform<S: AsRef<str>>(info: S) {
    // Info is a 1 lined help text of some kind of advice/tips/instructions
    // that is below the board and above the input prompt space.
    // Count how many println!() in display_board(). It should be at least one more than it
    print!("\x1B[{};1H", board::BOARD_HEIGHT_BY_CHAR + 2);

    // Erase the line, we want to clear any previous info so the user can see the new info
    print!("\x1B[2K{}", info.as_ref());
}

pub fn move_cursor_to_input() {
    // Should be at least one more below than the constant used in print_info()
    print!("\x1B[{};1H", board::BOARD_HEIGHT_BY_CHAR + 3);

    // Erase the line, we want to clear any previous input so the user can see his new input
    print!("\x1B[2K");
}

pub fn output<S: AsRef<str>>(output: S) {
    // Output shows if previous user command input was successful or not or any other info
    // Use this over other print() because this clears previous output.
    // Should be at least one more below than the constant used in move_cursor_to_input()
    print!("\x1B[{};1H", board::BOARD_HEIGHT_BY_CHAR + 4);

    // Erase the line, we want to clear any previous output so the user can see the new output
    // Do NOT move cursor at the end of this function to allow appending of info
    print!("\x1B[2K{}", output.as_ref());
}

pub fn clear_output() {
    print!("\x1B[{};1H\x1B[2K", board::BOARD_HEIGHT_BY_CHAR + 4);
}

pub fn terminal_bell() {
    print!("\x07");
}

pub fn clear_display() {
    // Clears the screen and puts the cursor at the very beginning (top left)
    // to redraw anything from the start (1st row 1st col), otherwise it may
    // redraw from the middle where it finished clearing previous texts
    print!("\x1B[2J");
    reset_cursor_to_start();
}

pub fn flush_buffer() {
    io::stdout().flush().unwrap();
}
