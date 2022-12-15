use board;
use colours;
use error;
use tiles::board_tile::PropertyStatus;

pub struct StreetTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
    pub colour: String, // Start of ANSI code (e.g. \x1b[41m)
    pub owner: Option<usize>,
    pub owner_colour: String,
    pub property_status: PropertyStatus,
    pub property_cost: i64,
    pub mortgage_value: i64,
    pub rent: i64,
    pub rent_levels: Vec<i64>,
    pub house_cost: i64,
    pub hotel_cost: i64,
}

impl StreetTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        let mut rent: Vec<i64> = Vec::<i64>::new();
        if let Some(rent_object) = tile_data["rent"].as_object() {
            rent_object
                .iter()
                .for_each(|(_, cost)| rent.push(cost.as_i64().unwrap()));
            rent.sort()
        }
        if rent.len() != 6 {
            panic!("{}", error::JSON_STREET_MISSING_RENT);
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
            house_cost: tile_data["house_cost"].as_i64().unwrap(),
            hotel_cost: tile_data["hotel_cost"].as_i64().unwrap(),
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

    pub fn update_rent_full_set(&mut self) {
        //! Only run this function if the rest of the set's tiles this tile is in
        //! is completely owned by one owner (i.e. this tile's owner has a monopoly
        //! on this tile's set). Rent update only applies to unimproved/non-mortgaged tiles.
        if let PropertyStatus::Owned = self.property_status {
            self.rent = self.rent_levels[0] * 2;
        }
        self.display_property_information();
    }

    pub fn update_rent(&mut self, rent: Option<i64>) {
        //! Mortgaging the property or buying and selling buildings affect the rent.
        //! Info for these actions are based on the <property_status> field
        //! We could implement this with fields for number of houses and hotels, but
        //! there would be more edge cases on which combinations would be rule breaking.
        //! It is easier to work with buildings as a tier: see board_tile::PropertyStatus
        if let Some(amount) = rent {
            self.rent = amount;
            return;
        }

        match self.property_status {
            PropertyStatus::Mortgaged => self.rent = 0,
            PropertyStatus::Unowned => self.rent = 0,
            PropertyStatus::Owned => self.rent = self.rent_levels[0],
            PropertyStatus::Tier1 => self.rent = self.rent_levels[1],
            PropertyStatus::Tier2 => self.rent = self.rent_levels[2],
            PropertyStatus::Tier3 => self.rent = self.rent_levels[3],
            PropertyStatus::Tier4 => self.rent = self.rent_levels[4],
            PropertyStatus::Tier5 => self.rent = self.rent_levels[5],
        }
    }

    pub fn display_tile_colour(&self) {
        print!(
            "\x1B[{1};{0}H", // {line_number};{character_col} in the terminal
            board::DISPLAY_BOARD_COORDS[self.id][0],
            board::DISPLAY_BOARD_COORDS[self.id][1],
        );
        print!("{}{}\x1b[0m", self.colour, board::TILE_COLOURED_REGION);
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
            PropertyStatus::Tier1 => print!("{}1\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier2 => print!("{}2\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier3 => print!("{}3\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier4 => print!("{}4\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier5 => print!("{}H\x1b[0m ${}", self.owner_colour, self.rent),
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

    pub fn display_card<'a>(&self, left_starting_col: usize, top_starting_row: usize) -> String {
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
            \n\x1B[{col}C  Rent{basic_rent: >29}  \
            \n\x1B[{col}C  Rent with full set{full_set_rent: >15}  \
            \n\x1B[{col}C  Rent with 1 house{one_house: >16}  \
            \n\x1B[{col}C  Rent with 2 houses{two_house: >15}  \
            \n\x1B[{col}C  Rent with 3 houses{three_house: >15}  \
            \n\x1B[{col}C  Rent with 4 houses{four_house: >15}  \
            \n\x1B[{col}C  Rent with hotel{hotel: >18}  \
            \n\x1B[{col}C{empty: ^CARD_WIDTH$}\
            \n\x1B[{col}C  House cost{house_cost: >23}  \
            \n\x1B[{col}C  Hotel cost{hotel_cost: >23}  \
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
            basic_rent = format!("${}", &self.rent_levels[0]),
            full_set_rent = format!("${}", &self.rent_levels[0] * 2),
            one_house = format!("${}", &self.rent_levels[1]),
            two_house = format!("${}", &self.rent_levels[2]),
            three_house = format!("${}", &self.rent_levels[3]),
            four_house = format!("${}", &self.rent_levels[4]),
            hotel = format!("${}", &self.rent_levels[5]),
            house_cost = format!("${} each", &self.house_cost),
            hotel_cost = format!("${} (after 4 houses)", &self.house_cost),
        )
    }
}
