use std::io::{self, Write};

use constants;

pub fn reset_cursor_to_start() {
    // Moves to the cursor to the very top left (1st row and 1st col)
    // Good default and standard location for cursor so we know to
    // always count the rows and cols from the top left
    print!("\x1B[1;1H");
}

pub fn reset_cursor_to_end() {
    // Count how many println!() in display_board(). It should be at least one more than it
    print!("\x1B[{};1H", constants::BOARD_HEIGHT_BY_CHAR + 2);
}

pub fn inform(info: &str) {
    // Info is a 1 lined help text of some kind of advice/tips/instructions
    // that is below the board and above the input prompt space.
    // Count how many println!() in display_board(). It should be at least one more than it
    print!("\x1B[{};1H", constants::BOARD_HEIGHT_BY_CHAR + 2);

    // Erase the line, we want to clear any previous info so the user can see the new info
    print!("\x1B[2K{info}");
}

pub fn move_cursor_to_input() {
    // Should be at least one more below than the constant used in print_info()
    print!("\x1B[{};1H", constants::BOARD_HEIGHT_BY_CHAR + 3);

    // Erase the line, we want to clear any previous input so the user can see his new input
    print!("\x1B[2K");
}

pub fn output(output: String) {
    // Output shows if previous user command input was successful or not or any other info
    // Use this over other print() because this clears previous output.
    // Should be at least one more below than the constant used in move_cursor_to_input()
    print!("\x1B[{};1H", constants::BOARD_HEIGHT_BY_CHAR + 4);

    // Erase the line, we want to clear any previous output so the user can see the new output
    print!("\x1B[2K{output}");
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
