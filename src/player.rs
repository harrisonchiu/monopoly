use constants;

// This also includes AI
#[derive(Clone, Copy, Debug)]
pub struct Player {
    // ID is unique int identifier of range [0, 3] (max 3 because a maximum of 4 players)
    // Also used to determined the position within the tile that it will be drawn onto.
    id: u8,
    avatar: char,
    // Could create a safely bounded custom integer type with a set range [0, N], but
    // position var is going to be used in index anyways, so it needs to be in usize type
    // Seems like unnecessary overhead to create type of small bounds only to cast to usize
    // Position on the board in terms of board indices [0, BOARD_TOTAL_NUMBER_OF_TILES]
    pub position: usize,
}

impl Player {
    pub fn new(id: u8, character: char) -> Self {
        Self {
            id: id,
            avatar: character,
            position: 0,
        }
    }

    pub fn move_forwards(&mut self, steps: u8) {
        // Loop the values around if it reaches past the upper bounds
        // Ex: 39 => 40 => 0 => 1 => ...
        // Can also use modulus, but this is faster and is just as readable
        // @steps: unsigned int [0, N], assumes value is at least 0
        let new_position: usize;
        let sum: usize = self.position + (steps as usize);
        if sum >= constants::BOARD_TOTAL_NUMBER_OF_TILES {
            new_position = sum - constants::BOARD_TOTAL_NUMBER_OF_TILES;
        } else {
            new_position = sum;
        }
        self.update_display_position(new_position);
        self.position = new_position;
    }

    pub fn move_backwards(&mut self, steps: u8) {
        // Loop the values around if it reaches past the lower bounds
        // Ex: 2 => 1 => 0 => 40 => 39 => ...
        // Can also use modulus, but this is faster and is just as readable
        // @steps: unsigned int [0, N], assumes value is at least 0
        let new_position: usize;
        let difference: usize = self.position - (steps as usize);
        if difference < constants::BOARD_TOTAL_NUMBER_OF_TILES {
            new_position = difference + constants::BOARD_TOTAL_NUMBER_OF_TILES;
        } else {
            new_position = difference;
        }
        self.update_display_position(new_position);
        self.position = new_position;
    }

    pub fn display_at_start(&self, start_position: usize) {
        print!(
            // {line};{col} in terminal; space (at end after H) erases previous avatar
            "\x1B[{1};{0}H{2}",
            // Display players as |0 1 2 3| based on id, assuming 7 character wide tiles
            constants::DISPLAY_BOARD_COORDS[start_position][0] + (2 * self.id as u8),
            constants::DISPLAY_BOARD_COORDS[start_position][1] + 2, // 3rd row of tile
            self.avatar
        );
    }

    fn update_display_position(&self, new_position: usize) {
        // This fn must be used BEFORE updating position because it needs to know the previous
        // position to erase the previous avatar, showing a "move" and stopping duplicate avatars
        print!(
            // {line};{col} in terminal; space (at end after H) erases previous avatar
            "\x1B[{1};{0}H ",
            constants::DISPLAY_BOARD_COORDS[self.position][0] + (2 * self.id as u8),
            constants::DISPLAY_BOARD_COORDS[self.position][1] + 2 // 3rd row of tile
        );
        print!(
            // {line};{col} in terminal
            "\x1B[{1};{0}H{2}",
            // Display players as |0 1 2 3| based on id, assuming 7 character wide tiles
            constants::DISPLAY_BOARD_COORDS[new_position][0] + (2 * self.id as u8),
            constants::DISPLAY_BOARD_COORDS[new_position][1] + 2, // 3rd row of tile
            self.avatar // Draw avatar in new location to illustrate players moving
        );
    }
}
