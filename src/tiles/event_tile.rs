use board;

pub struct EventTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
    pub colour: String,
}

impl EventTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        Self {
            id: id,
            name: tile_data["name"].as_str().unwrap().to_string(),
            set_name: tile_data["set"].as_str().unwrap().to_string(),
            colour: "".to_string(),
        }
    }

    pub fn clear_and_goto_line(&self, line: u8) {
        print!(
            // {line};{col}, the space erases previous info so text does not overlap
            "\x1B[{1};{0}H{2:^tile_width$}\x1B[{1};{0}H", // resets the cursor to draw
            board::DISPLAY_BOARD_COORDS[self.id][0],
            // Line 1: colour, line 2: property info, line 3: player positions
            // We subtract 1 because DISPLAY_BOARD_COORDS is already at line 1, so no need to add
            board::DISPLAY_BOARD_COORDS[self.id][1] + line - 1,
            " ",
            tile_width = board::TILE_LENGTH_BY_CHAR
        )
    }

    pub fn display_tile_id(&self) {
        self.clear_and_goto_line(3);

        print!(
            "{:^tile_width$}",
            self.id,
            tile_width = board::TILE_LENGTH_BY_CHAR
        );
    }

    pub fn display_card(&self, left_starting_col: usize, top_starting_row: usize) -> String {
        const CARD_WIDTH: usize = 37;
        const WHITE_BACKGROUND: &str = "\x1b[47m";
        const BLACK_FOREGROUND: &str = "\x1b[30m";

        // idk why one space is needed after H first line but it starts earlier
        format!(
            "\x1B[{row};{col}H {WHITE_BACKGROUND}{set_colour}{empty: ^CARD_WIDTH$}\x1b[0m\
            \n\x1B[{col}C{WHITE_BACKGROUND}{BLACK_FOREGROUND}{empty: ^CARD_WIDTH$}\
            \n\x1B[{col}C{name: ^CARD_WIDTH$}\
            \n\x1B[{col}C{empty: ^CARD_WIDTH$}\
            \x1b[0m",
            empty = " ",
            row = top_starting_row,
            col = left_starting_col,
            set_colour = match self.colour.is_empty() {
                true => WHITE_BACKGROUND,
                false => &self.colour,
            },
            name = &self.name,
        )
    }
}
