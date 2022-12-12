use std::collections::{HashMap, HashSet};
use std::iter::{repeat, FromIterator};
type PropertySet = HashMap<String, HashSet<usize>>;
type OwnershipRecords = Vec<PropertySet>;

use board::Board;
use error;
use player::{Pieces, Player};
use tiles::{
    board_tile::BoardTile, event_tile::EventTile, railroad_tile::RailroadTile,
    street_tile::StreetTile, utility_tile::UtilityTile,
};

use rand::distributions::{Distribution, Uniform};
use rand::{rngs::StdRng, SeedableRng};

pub struct Monopoly {
    board: Board,
    players: Vec<Player>,
    dice: [StdRng; 2],
    dice_range: Uniform<i8>,

    /// { "Blue": {39, 37}, "Brown": {2, 3}, ... } // store tiles by their id
    /// So we know that tiles 39 and 37 belong in the same set together called "Blue"
    property_sets: PropertySet,

    /// [
    ///     { "Blue": {39}, "Brown": {1,3} }, // Player 1 (by index, id = 0)
    ///     { "Cyan": {6,8}, "Blue": {37}, ...}, // Player 2 (by index, id = 1)
    /// ]
    /// Keeps track of which player owns which tiles according to their sets,
    /// organized in this way so it is easy to tell if a player completes their set
    ownership_records: OwnershipRecords,
}

impl Monopoly {
    pub fn new(number_of_players: usize) -> Self {
        let tile_data_json: &str = include_str!("./tiles/board_tile_data.json");
        let json: serde_json::Value =
            serde_json::from_str(&tile_data_json).expect(&format!("{}", error::JSON_DESERIALIZE));

        error::validate_tile_data_json(&json, true);

        // Skip first JSON object because it is documentation and metadata, create board
        // with the rest of it. JSON is array of objects so it should preserve order; it should
        // define all the board tiles from GO (start) to Boardwalk (last tile before GO) in order
        let mut tiles: Vec<BoardTile> = Vec::<BoardTile>::new();
        let mut sets: PropertySet = PropertySet::new();
        for (id, tile_data) in json.as_array().unwrap().iter().skip(1).enumerate() {
            sets.entry(tile_data["set"].to_string())
                .or_insert(HashSet::from([id]))
                .insert(id);

            match tile_data["type"].as_str().unwrap() {
                "Street" => tiles.push(BoardTile::Street(StreetTile::new(id, tile_data))),
                "Railroad" => tiles.push(BoardTile::Railroad(RailroadTile::new(id, tile_data))),
                "Utility" => tiles.push(BoardTile::Utility(UtilityTile::new(id, tile_data))),
                "Event" => tiles.push(BoardTile::Event(EventTile::new(id, tile_data))),
                _ => continue,
            }
        }

        // Instantialize the players
        let mut players: Vec<Player> = Vec::<Player>::with_capacity(number_of_players);
        for player_id in 0..4 {
            players.push(Player::new(player_id));
        }

        Self {
            board: Board::new(tiles),
            players: players,
            dice: [StdRng::from_entropy(), StdRng::from_entropy()],
            dice_range: Uniform::new_inclusive(1, 6),
            property_sets: sets,
            ownership_records: {
                OwnershipRecords::from_iter(repeat(HashMap::new()).take(number_of_players))
            },
        }
    }

    pub fn display_game(&self) {
        // May need to use this to display other things e.g. logs, inventory, etc.
        self.board.display_board();
        self.players
            .iter()
            .for_each(|player| player.display_at_position(player.position));
    }

    // Getters for most fields of BoardTile structs
    pub fn get_tile_name(&self, position: usize) -> &String {
        match self.board.get_tile(position) {
            BoardTile::Street(property) => &property.name,
            BoardTile::Railroad(property) => &property.name,
            BoardTile::Utility(property) => &property.name,
            BoardTile::Event(tile) => &tile.name,
        }
    }

    pub fn get_set_name(&self, position: usize) -> &String {
        match self.board.get_tile(position) {
            BoardTile::Street(property) => &property.set_name,
            BoardTile::Railroad(property) => &property.set_name,
            BoardTile::Utility(property) => &property.set_name,
            BoardTile::Event(tile) => &tile.set_name,
        }
    }

    pub fn get_colour(&self, position: usize) -> &String {
        match self.board.get_tile(position) {
            BoardTile::Street(property) => &property.colour,
            BoardTile::Railroad(property) => &property.colour,
            BoardTile::Utility(property) => &property.colour,
            BoardTile::Event(tile) => &tile.colour,
        }
    }

    pub fn get_owner(&self, tile: usize) -> Option<usize> {
        match self.board.get_tile(tile) {
            BoardTile::Street(property) => property.owner,
            BoardTile::Railroad(property) => property.owner,
            BoardTile::Utility(property) => property.owner,
            BoardTile::Event(_) => None, // EventTiles has no owner
        }
    }

    pub fn get_owner_colour(&self, tile: usize) -> &String {
        match self.board.get_tile(tile) {
            BoardTile::Street(property) => &property.owner_colour,
            BoardTile::Railroad(property) => &property.owner_colour,
            BoardTile::Utility(property) => &property.owner_colour,
            BoardTile::Event(tile) => &tile.colour,
        }
    }

    pub fn get_property_cost(&self, position: usize) -> i64 {
        match self.board.get_tile(position) {
            BoardTile::Street(property) => property.property_cost,
            BoardTile::Railroad(property) => property.property_cost,
            BoardTile::Utility(property) => property.property_cost,
            BoardTile::Event(_) => 0,
        }
    }

    pub fn get_rent(&self, tile: usize, dice: &[i8; 2]) -> i64 {
        match self.board.get_tile(tile) {
            BoardTile::Street(property) => property.rent,
            BoardTile::Railroad(property) => property.rent,
            BoardTile::Utility(property) => property.rent_multiplier * (dice[0] + dice[1]) as i64,
            BoardTile::Event(_) => 0, // EventTiles has no owner
        }
    }

    pub fn get_player(&self, id: usize) -> &Player {
        &self.players[id]
    }

    // Actions player can do in the game
    pub fn roll_dice(&mut self) -> [i8; 2] {
        [
            self.dice_range.sample(&mut self.dice[0]),
            self.dice_range.sample(&mut self.dice[1]),
        ]
    }

    pub fn is_doubles(&self, dice: &[i8; 2]) -> bool {
        dice[0] == dice[1]
    }

    pub fn move_player(&mut self, player: usize, dice: &[i8; 2]) -> usize {
        self.players[player].walk(dice[0] + dice[1])
    }

    pub fn pay_rent_to(&mut self, tenent: usize, landlord: usize, rent: i64) {
        self.players[tenent].pay(rent);
        self.players[landlord].collect(rent);
    }

    pub fn buy_tile(&mut self, buyer: usize, tile: usize) -> Option<usize> {
        let set_name: String = self.get_set_name(tile).to_string();

        match self.board.get_tile_mut(tile) {
            BoardTile::Street(property) if property.owner.is_none() => {
                let mut is_set_complete: bool = false;
                let property_cost: i64 = property.property_cost;

                // Record the ownership in the records before transfering ownership to buyer
                // in order to check if the buyer completed the set by looking at the records,
                // so we can adjust and update the rent as we acquire it.
                self.ownership_records[buyer]
                    .entry(set_name.to_string())
                    .or_insert(HashSet::from([tile]))
                    .insert(tile);

                // Check if the player has a completed colour set after acquiring this
                if let Some(set) = self.ownership_records[buyer].get(&set_name) {
                    is_set_complete = set.len() == self.property_sets[&set_name].len();
                }

                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &Pieces::colour(&buyer).to_string());

                // Apply the double rent rule on full set to the other tiles of that set
                if is_set_complete {
                    for tile_in_set in &self.property_sets[&set_name] {
                        match self.board.get_tile_mut(*tile_in_set) {
                            BoardTile::Street(t) => t.update_rent_full_set(),
                            _ => panic!("Somehow a different tile type got mixed with this set"),
                        }
                    }
                }

                self.players[buyer].pay(property_cost);
                Some(tile)
            }
            BoardTile::Railroad(property) if property.owner.is_none() => {
                let owned_railroads: &HashSet<usize>;
                let property_cost: i64 = property.property_cost;

                // Record the ownership in the records before transfering ownership to buyer
                // in order to check if the buyer completed the set by looking at the records,
                // so we can adjust and update the rent as we acquire it.
                self.ownership_records[buyer]
                    .entry(set_name.to_string())
                    .or_insert(HashSet::from([tile]))
                    .insert(tile);

                // Check if the player has a completed colour set after acquiring this
                if let Some(set) = self.ownership_records[buyer].get(&set_name) {
                    owned_railroads = set;
                } else {
                    panic!("Could not purchase Railroad tile. Failed to record ownership of tile.");
                }

                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &Pieces::colour(&buyer).to_string());

                // Rent should scale for all tiles of this set based on the number of it owned
                for tile_in_set in owned_railroads {
                    match self.board.get_tile_mut(*tile_in_set) {
                        BoardTile::Railroad(t) => {
                            t.update_rent_total_number_of_owned_railroads(owned_railroads.len());
                        }
                        _ => panic!("Somehow a different tile type got mixed with this set"),
                    }
                }

                self.players[buyer].pay(property_cost);
                Some(tile)
            }
            BoardTile::Utility(property) if property.owner.is_none() => {
                let owned_utilities: &HashSet<usize>;
                let property_cost: i64 = property.property_cost;

                // Record the ownership in the records before transfering ownership to buyer
                // in order to check if the buyer completed the set by looking at the records,
                // so we can adjust and update the rent as we acquire it.
                self.ownership_records[buyer]
                    .entry(set_name.to_string())
                    .or_insert(HashSet::from([tile]))
                    .insert(tile);

                // Check if the player has a completed colour set after acquiring this
                if let Some(set) = self.ownership_records[buyer].get(&set_name) {
                    owned_utilities = set;
                } else {
                    panic!("Could not purchase Utility tile. Failed to record ownership of tile.");
                }

                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &Pieces::colour(&buyer).to_string());

                // Rent should scale for all tiles of this set based on the number of it owned
                for tile_in_set in owned_utilities {
                    match self.board.get_tile_mut(*tile_in_set) {
                        BoardTile::Utility(t) => {
                            t.update_rent_total_number_of_owned_utilities(owned_utilities.len());
                        }
                        _ => panic!("Somehow a different tile type got mixed with this set"),
                    }
                }

                self.players[buyer].pay(property_cost);
                Some(tile)
            }
            _ => None, // EventTiles cannot be purchased
        }
    }
}
