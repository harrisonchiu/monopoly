use board;

pub struct Pieces;

impl Pieces {
    // The return output has the same lifetime as <id> parameter
    pub fn colour<'a>(id: &'a usize) -> &'a str {
        match id {
            0 => "\x1b[96m",
            1 => "\x1b[92m",
            2 => "\x1b[91m",
            3 => "\x1b[95m",
            _ => " ",
        }
    }

    pub fn avatar<'a>(id: &'a usize) -> &'a char {
        match id {
            0 => &'A',
            1 => &'B',
            2 => &'C',
            3 => &'D',
            _ => &'?',
        }
    }

    pub fn view_ids<'a>() -> String {
        format!(
            "{}: ID 0 // {}: ID 1 // {}: ID 2 // {}: ID 3",
            Self::avatar(&0),
            Self::avatar(&1),
            Self::avatar(&2),
            Self::avatar(&3)
        )
    }
}

pub struct Player {
    // ID is unique int identifier of range [0, 3] (max 3 because a maximum of 4 players)
    // Used to find the position within the tile that it will be drawn onto.
    // Also used to find a player in the list of players (done because id == index in list)
    // Type of usize to easily use as index in arrays and vectors
    pub id: usize,
    pub avatar: char,
    pub colour: String,
    // Could create a safely bounded custom integer type with a set range [0, N], but
    // position var is going to be used in index anyways, so it needs to be in usize type
    // Seems like unnecessary overhead to create type of small bounds only to cast to usize
    // Position on the board in terms of board indices [0, BOARD_TOTAL_NUMBER_OF_TILES]
    pub position: usize,
    pub money: i64,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            id: id,
            avatar: Pieces::avatar(&id).clone(),
            colour: Pieces::colour(&id).to_string(),
            position: 0,
            money: 10000,
        }
    }

    pub fn pay(&mut self, amount: i64) {
        self.money -= amount;
    }

    pub fn collect(&mut self, amount: i64) {
        self.money += amount;
    }

    pub fn walk(&mut self, steps: i8) -> usize {
        // Loop the values around if it reaches past the upper or lower bounds
        // Ex: 38 => 39 => 0 => 1 => ... and 1 => 0 => 39 => 38
        // Can also use modulus, but this is faster and is just as readable
        let new_position: i8;
        let position: i8 = self.position as i8 + steps;
        if position >= board::BOARD_TOTAL_NUMBER_OF_TILES as i8 {
            new_position = position - board::BOARD_TOTAL_NUMBER_OF_TILES as i8;
        } else if position < 0 {
            new_position = position + board::BOARD_TOTAL_NUMBER_OF_TILES as i8;
        } else {
            new_position = position;
        }
        self.display_at_position(new_position as usize);
        self.position = new_position as usize;
        self.position
    }

    pub fn display_at_position(&self, position: usize) {
        //! Erases the avatar at the previous location and draws it at the given location
        //! This shows the player "moved" and prevents duplicate avatars from showing
        print!(
            // {line};{col} in terminal; space (at end after H) erases previous avatar
            "\x1B[{1};{0}H ",
            board::DISPLAY_BOARD_COORDS[self.position][0] + (2 * self.id as u8),
            board::DISPLAY_BOARD_COORDS[self.position][1] + 2 // 3rd row of tile
        );
        print!(
            // {line};{col} in terminal
            "\x1B[{1};{0}H{2}{3}\x1b[0m",
            // Display players as |0 1 2 3| based on id, assuming 7 character wide tiles
            board::DISPLAY_BOARD_COORDS[position][0] + (2 * self.id as u8),
            board::DISPLAY_BOARD_COORDS[position][1] + 2, // 3rd row of tile
            self.colour,
            self.avatar // Draw avatar in new location to illustrate players moving
        );
    }
}
