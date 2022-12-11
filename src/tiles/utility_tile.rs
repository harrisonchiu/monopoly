use board;
use colours;
use error;
use tiles::board_tile::PropertyStatus;

pub struct UtilityTile {
    pub id: usize,
    pub name: String,
    pub set_name: String,
    pub colour: String, // Start of ANSI code (e.g. \x1b[41m)
    pub owner: Option<usize>,
    pub owner_colour: String,
    pub property_status: PropertyStatus,
    pub property_cost: i64,
    pub rent_multiplier: i64,
    pub rent_multiplier_levels: Vec<i64>,
}

impl UtilityTile {
    pub fn new(id: usize, tile_data: &serde_json::Value) -> Self {
        let mut rent_multiplier_levels: Vec<i64> = Vec::<i64>::new();
        if let Some(rent_object) = tile_data
            .get("rent_multiplier")
            .expect(error::JSON_MISSING_RENT)
            .as_object()
        {
            rent_object.iter().for_each(|(_, cost)| {
                rent_multiplier_levels.push(cost.as_i64().expect(error::JSON_DESERIALIZE_TO_I64))
            });
            rent_multiplier_levels.sort();
        }
        if rent_multiplier_levels.len() != 2 {
            panic!("Utility rent field needs to be an object with 2 int rent cost values")
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
            rent_multiplier: rent_multiplier_levels
                .get(0)
                .expect(error::JSON_MISSING_RENT)
                .clone(),
            rent_multiplier_levels: rent_multiplier_levels,
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

    fn update_rent(&mut self, multiplier: Option<i64>) {
        //! Mortgaging the property or buying and selling buildings affect the rent.
        //! Info for these actions are based on the <property_status> field
        //! We could implement this with fields for number of houses and hotels, but
        //! there would be more edge cases on which combinations would be rule breaking.
        //! It is easier to work with buildings as a tier: see board_tile::PropertyStatus
        if let Some(amount) = multiplier {
            self.rent_multiplier = amount
        }

        match self.property_status {
            PropertyStatus::Mortgaged => self.rent_multiplier = 0,
            PropertyStatus::Unowned => self.rent_multiplier = 0,
            PropertyStatus::Owned => self.rent_multiplier = self.rent_multiplier_levels[0],
            PropertyStatus::Tier1 => self.rent_multiplier = self.rent_multiplier_levels[1],
            _ => println!("Breaking the rules likely due to error in game logic"),
        }
    }

    pub fn update_rent_total_number_of_owned_utilities(&mut self, utilities: usize) {
        match utilities {
            0 => self.property_status = PropertyStatus::Unowned,
            1 => self.property_status = PropertyStatus::Owned,
            2 => self.property_status = PropertyStatus::Tier1,
            _ => panic!(
                "When updating total number of owned utilities, an impossible \
                number of owned utilities was used!"
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
            PropertyStatus::Owned => print!("{}X\x1b[0m x{}", self.colour, self.rent_multiplier),
            PropertyStatus::Tier1 => print!("{}X\x1b[0m x{}", self.colour, self.rent_multiplier),
            _ => print!(" ERROR "),
        }
    }
}
