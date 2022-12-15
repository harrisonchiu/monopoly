use board;
use colours;
use error;
use tiles::board_tile::PropertyStatus;

pub struct RailroadTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
    pub colour: String,
    pub owner: Option<usize>,
    pub owner_colour: String,
    pub property_status: PropertyStatus,
    pub property_cost: i64,
    pub mortgage_value: i64,
    pub rent: i64,
    pub rent_levels: Vec<i64>,
}

impl RailroadTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        let mut rent: Vec<i64> = Vec::<i64>::new();
        if let Some(rent_object) = tile_data["rent"].as_object() {
            rent_object
                .iter()
                .for_each(|(_, cost)| rent.push(cost.as_i64().unwrap()));
            rent.sort();
        }
        if rent.len() != 4 {
            panic!("{}", error::JSON_RAILROAD_MISSING_RENT);
        }

        Self {
            id: id,
            name: tile_data["name"].as_str().unwrap().to_string(),
            set_name: tile_data["set"].as_str().unwrap().to_string(),
            colour: colours::bg_set_colour(tile_data["set"].as_str().unwrap()),
            owner: None,
            owner_colour: "".to_string(),
            property_status: PropertyStatus::Unowned,
            property_cost: tile_data["property_cost"].as_i64().unwrap(),
            mortgage_value: tile_data["mortgage_value"].as_i64().unwrap(),
            rent: rent[0],
            rent_levels: rent,
        }
    }

    pub fn acquired_by(&mut self, owner: usize, owner_colour: &String) {
        self.owner = Some(owner);
        self.owner_colour = owner_colour.to_string();

        // If purchased, then own it; if traded, keep property status
        if self.property_status == PropertyStatus::Unowned {
            self.property_status = PropertyStatus::Owned;
        }

        // Basic update of rent update based on property status
        // Does NOT check for full set ownership and its rent rules
        self.update_rent(None);
        self.display_property_information();
    }

    fn update_rent(&mut self, rent: Option<i64>) {
        //! Mortgaging the property or buying and selling buildings affect the rent.
        //! Info for these actions are based on the <property_status> field
        //! We could implement this with fields for number of houses and hotels, but
        //! there would be more edge cases on which combinations would be rule breaking.
        //! It is easier to work with buildings as a tier: see board_tile::PropertyStatus
        if let Some(amount) = rent {
            self.rent = amount
        }

        match self.property_status {
            PropertyStatus::Mortgaged => self.rent = 0,
            PropertyStatus::Unowned => self.rent = 0,
            PropertyStatus::Owned => self.rent = self.rent_levels[0],
            PropertyStatus::Tier1 => self.rent = self.rent_levels[1],
            PropertyStatus::Tier2 => self.rent = self.rent_levels[2],
            PropertyStatus::Tier3 => self.rent = self.rent_levels[3],
            _ => println!("Breaking the rules likely due to error in game logic"),
        }
    }

    pub fn update_rent_total_number_of_owned_railroads(&mut self, railroads: usize) {
        match railroads {
            0 => self.property_status = PropertyStatus::Unowned,
            1 => self.property_status = PropertyStatus::Owned,
            2 => self.property_status = PropertyStatus::Tier1,
            3 => self.property_status = PropertyStatus::Tier2,
            4 => self.property_status = PropertyStatus::Tier3,
            _ => panic!(
                "When updating total number of owned railroads, an impossible \
                number of owned railroads was used!"
            ),
        }

        self.update_rent(None);
        self.display_property_information();
    }

    pub fn display_tile_colour(&self) {
        print!(
            "\x1B[{1};{0}H{2}{3}", // {line_number};{character_col} in the terminal
            board::DISPLAY_BOARD_COORDS[self.id][0],
            board::DISPLAY_BOARD_COORDS[self.id][1],
            self.colour,
            board::TILE_COLOURED_REGION
        );
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

    pub fn display_property_information(&self) {
        self.clear_and_goto_line(2);

        match self.property_status {
            PropertyStatus::Mortgaged => print!("|MRTGAGE|"),
            PropertyStatus::Unowned => print!(" ${}", self.property_cost),
            // TODO: Somehow get player colour from owner_id,
            // rn this is just the tile's colour and not the owner
            PropertyStatus::Owned => print!("{}X\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier1 => print!("{}X\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier2 => print!("{}X\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier3 => print!("{}X\x1b[0m ${}", self.owner_colour, self.rent),
            _ => print!(" ERROR "),
        }
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
            \n\x1B[{col}C{set_colour}{empty: ^CARD_WIDTH$}\x1b[0m\
            \n\x1B[{col}C{WHITE_BACKGROUND}{BLACK_FOREGROUND}{empty: ^CARD_WIDTH$}\
            \n\x1B[{col}C{name: ^CARD_WIDTH$}\
            \n\x1B[{col}C{empty: ^CARD_WIDTH$}\
            \n\x1B[{col}C  Property cost{property_cost: >20}  \
            \n\x1B[{col}C  Mortgage value{mortgage_value: >19}  \
            \n\x1B[{col}C{empty: ^CARD_WIDTH$}\
            \n\x1B[{col}C  If 1 owned in set{rent_level_1: >16}  \
            \n\x1B[{col}C  If 2 owned in set{rent_level_2: >16}  \
            \n\x1B[{col}C  If 3 owned in set{rent_level_3: >16}  \
            \n\x1B[{col}C  If 4 owned in set{rent_level_4: >16}  \
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
            property_cost = format!("${}", &self.property_cost),
            mortgage_value = format!("${}", &self.mortgage_value),
            rent_level_1 = format!("${}", &self.rent_levels[0]),
            rent_level_2 = format!("${}", &self.rent_levels[1]),
            rent_level_3 = format!("${}", &self.rent_levels[2]),
            rent_level_4 = format!("${}", &self.rent_levels[3]),
        )
    }
}
