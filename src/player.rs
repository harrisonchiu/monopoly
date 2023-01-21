use colour::Colour;

pub struct Avatar {
    character: char,
    colour: String,
    avatar: String,
}

impl Avatar {
    pub fn new(id: i8) -> Self {
        let character: char = Self::get_character(id);
        let colour: String = Self::get_colour(id).to_string();
        let avatar: String = Self::get_avatar(id);

        Self {
            character,
            colour,
            avatar,
        }
    }

    pub fn get_character(id: i8) -> char {
        //! Static method to get info of a player's Avatar without refering to the Player struct
        //! Consider this like a factory to build the Avatar based on the player id
        match id {
            -1 => ' ',
            0 => 'A',
            1 => 'B',
            2 => 'C',
            3 => 'D',
            _ => '?',
        }
    }

    pub fn get_colour<'a>(id: i8) -> &'a str {
        //! Static method to get info of a player's Avatar without refering to the Player struct
        //! Consider this like a factory to build the Avatar based on the player id
        match id {
            -1 => Colour::foreground(""),
            0 => Colour::foreground("Bright Cyan"),
            1 => Colour::foreground("Bright Green"),
            2 => Colour::foreground("Bright Red"),
            3 => Colour::foreground("Bright Magenta"),
            _ => Colour::foreground(""),
        }
    }

    pub fn get_avatar<'a>(id: i8) -> String {
        format!("{}{}\x1b[0m", Self::get_colour(id), Self::get_character(id))
    }

    pub fn get_all_avatars<'a>(players: &[bool]) -> String {
        format!(
            "{} {} {} {}",
            if players[0] {
                Avatar::get_avatar(0)
            } else {
                Avatar::get_avatar(-1)
            },
            if players[1] {
                Avatar::get_avatar(1)
            } else {
                Avatar::get_avatar(-1)
            },
            if players[2] {
                Avatar::get_avatar(2)
            } else {
                Avatar::get_avatar(-1)
            },
            if players[3] {
                Avatar::get_avatar(3)
            } else {
                Avatar::get_avatar(-1)
            }
        )
    }
}

pub struct Player {
    pub id: i8,
    pub avatar: Avatar,
    pub position: usize,
    pub last_position: usize,
    money: i64,
}

impl Player {
    const NUMBER_OF_TILES: i8 = 40;

    pub fn new(id: i8) -> Self {
        Self {
            id,
            avatar: Avatar::new(id),
            position: 0,
            last_position: 0,
            money: 9000,
        }
    }

    pub fn walk(&mut self, steps: i8) {
        // Loop the values around if it reaches past the upper or lower bounds: [0, 40]
        // Ex: 38 => 39 => 0 => 1 => ... and 1 => 0 => 39 => 38
        // Can also use modulus, but this is faster and is just as readable
        let new_position: i8;
        let position: i8 = self.position as i8 + steps;
        if position >= Self::NUMBER_OF_TILES {
            new_position = position - Self::NUMBER_OF_TILES;
        } else if position < 0 {
            new_position = position + Self::NUMBER_OF_TILES;
        } else {
            new_position = position;
        }

        self.last_position = self.position;
        self.position = new_position as usize;
    }
}

impl std::fmt::Display for Avatar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.avatar)
    }
}
