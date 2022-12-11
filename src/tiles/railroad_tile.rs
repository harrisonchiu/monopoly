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
            name: tile_data["name"].to_string(),
            set_name: tile_data["set"].to_string(),
            colour: colours::get_set_background_colour(tile_data["set"].as_str().unwrap()),
            owner: None,
            owner_colour: "".to_string(),
            property_status: PropertyStatus::Unowned,
            property_cost: tile_data["property_cost"].as_i64().unwrap(),
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

    pub fn display_property_information(&self) {
        print!(
            // {line};{col}, the space erases previous info so text does not overlap
            "\x1B[{1};{0}H       \x1B[{1};{0}H", // resets the cursor to draw
            board::DISPLAY_BOARD_COORDS[self.id][0],
            board::DISPLAY_BOARD_COORDS[self.id][1] + 1, // 2nd row of tile
        );

        match self.property_status {
            PropertyStatus::Mortgaged => print!("|MRTGAGE|"),
            PropertyStatus::Unowned => print!(" ${}", self.property_cost),
            // TODO: Somehow get player colour from owner_id,
            // rn this is just the tile's colour and not the owner
            PropertyStatus::Owned => print!("{}X\x1b[0m ${}", self.colour, self.rent),
            PropertyStatus::Tier1 => print!("{}X\x1b[0m ${}", self.colour, self.rent),
            PropertyStatus::Tier2 => print!("{}X\x1b[0m ${}", self.colour, self.rent),
            PropertyStatus::Tier3 => print!("{}X\x1b[0m ${}", self.colour, self.rent),
            _ => print!(" ERROR "),
        }
    }
}
