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
    pub rent: i64,
    pub rent_levels: Vec<i64>,
}

impl StreetTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        let mut rent: Vec<i64> = Vec::<i64>::new();
        if let Some(rent_object) = tile_data
            .get("rent")
            .expect(error::JSON_MISSING_RENT)
            .as_object()
        {
            rent_object.iter().for_each(|(_, cost)| {
                rent.push(cost.as_i64().expect(error::JSON_DESERIALIZE_TO_I64))
            });
            rent.sort()
        }
        if rent.len() != 6 {
            panic!("Street rent field needs to be an object with 6 int rent cost values")
        }

        Self {
            id: id,
            name: tile_data
                .get("name")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
            set_name: tile_data
                .get("set")
                .expect(error::JSON_MISSING_NAME)
                .to_string(),
            colour: colours::get_background_colour_from_set(
                tile_data
                    .get("set")
                    .expect(error::JSON_MISSING_NAME)
                    .as_str()
                    .expect(error::JSON_DESERIALIZE_TO_STR),
            ),
            owner: None,
            owner_colour: "".to_string(),
            property_status: PropertyStatus::Unowned,
            property_cost: tile_data
                .get("property_cost")
                .expect(error::JSON_MISSING_PROPERTY_COST)
                .as_i64()
                .expect(error::JSON_DESERIALIZE_TO_I64),
            rent: rent.get(0).expect(error::JSON_MISSING_RENT).clone(),
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
            PropertyStatus::Owned => print!("{}X\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier1 => print!("{}1\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier2 => print!("{}2\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier3 => print!("{}3\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier4 => print!("{}4\x1b[0m ${}", self.owner_colour, self.rent),
            PropertyStatus::Tier5 => print!("{}H\x1b[0m ${}", self.owner_colour, self.rent),
        }
    }
}
