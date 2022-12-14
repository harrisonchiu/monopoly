use std::collections::{BTreeMap, HashSet};
use std::iter::{repeat, FromIterator};
pub type PropertySet = BTreeMap<String, HashSet<usize>>;

use board::Board;
use error;
use interface;
use player::Player;
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
    ownership_records: Vec<PropertySet>,

    pub proposer: usize,
    proposer_money: i64,
    proposer_tiles: BTreeMap<usize, String>,

    pub receiver: usize,
    receiver_money: i64,
    receiver_tiles: BTreeMap<usize, String>,
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
        let mut all_sets_with_their_tiles: PropertySet = PropertySet::new();
        let mut ownable_sets_empty: PropertySet = PropertySet::new();
        for (id, tile_data) in json.as_array().unwrap().iter().skip(1).enumerate() {
            all_sets_with_their_tiles
                .entry(tile_data["set"].as_str().unwrap().to_string())
                .or_insert(HashSet::from([id]))
                .insert(id);

            match tile_data["type"].as_str().unwrap() {
                "Street" => {
                    tiles.push(BoardTile::Street(StreetTile::new(id, tile_data)));
                    ownable_sets_empty
                        .entry(tile_data["set"].as_str().unwrap().to_string())
                        .or_insert(HashSet::new());
                }
                "Railroad" => {
                    tiles.push(BoardTile::Railroad(RailroadTile::new(id, tile_data)));
                    ownable_sets_empty
                        .entry(tile_data["set"].as_str().unwrap().to_string())
                        .or_insert(HashSet::new());
                }
                "Utility" => {
                    tiles.push(BoardTile::Utility(UtilityTile::new(id, tile_data)));
                    ownable_sets_empty
                        .entry(tile_data["set"].as_str().unwrap().to_string())
                        .or_insert(HashSet::new());
                }
                "Event" => tiles.push(BoardTile::Event(EventTile::new(id, tile_data))),
                _ => continue,
            }
        }

        // Instantialize the players
        let mut players: Vec<Player> = Vec::<Player>::with_capacity(number_of_players);
        for player_id in 0..number_of_players {
            players.push(Player::new(player_id));
        }

        Self {
            board: Board::new(tiles),
            players: players,
            dice: [StdRng::from_entropy(), StdRng::from_entropy()],
            dice_range: Uniform::new_inclusive(1, 6),
            property_sets: all_sets_with_their_tiles,
            ownership_records: Vec::<PropertySet>::from_iter(
                repeat(ownable_sets_empty).take(number_of_players),
            ),
            proposer: 0,
            proposer_money: 0,
            proposer_tiles: BTreeMap::new(),
            receiver: 0,
            receiver_money: 0,
            receiver_tiles: BTreeMap::new(),
        }
    }

    pub fn display_game(&self) {
        // May need to use this to display other things e.g. logs, inventory, etc.
        self.board.display_board();
        self.display_players();
        self.update_inventory_display();
    }

    pub fn update_inventory_display(&self) {
        interface::display_inventory(&self.ownership_records, &self.players);
    }

    pub fn view_tile_ids(&self) {
        // display_tile_id() already clears the line to print its id
        for tile in &self.board.board {
            match tile {
                BoardTile::Street(property) => property.display_tile_id(),
                BoardTile::Railroad(property) => property.display_tile_id(),
                BoardTile::Utility(property) => property.display_tile_id(),
                BoardTile::Event(tile) => tile.display_tile_id(),
            }
        }
    }

    pub fn display_players(&self) {
        // Need to clear any duplicate players or display ids, in order to redraw the players
        for tile in &self.board.board {
            match tile {
                BoardTile::Street(property) => property.clear_and_goto_line(3),
                BoardTile::Railroad(property) => property.clear_and_goto_line(3),
                BoardTile::Utility(property) => property.clear_and_goto_line(3),
                BoardTile::Event(tile) => tile.clear_and_goto_line(3),
            }
        }
        self.players
            .iter()
            .for_each(|player| player.display_at_position(player.position));
    }

    pub fn display_full_tile_info(&self, tile: usize) {
        interface::display_board_tile(self.board.get_tile(tile));
    }

    pub fn display_trading_desk(&self) {
        interface::display_trading_desk(
            self.players[self.proposer].avatar,
            self.proposer_money,
            &self.proposer_tiles,
            self.players[self.receiver].avatar,
            self.receiver_money,
            &self.receiver_tiles,
        );
    }

    /* Getters for most of each board tile struct's fields */
    pub fn get_tile_name(&self, position: usize) -> String {
        match self.board.get_tile(position) {
            BoardTile::Street(property) => property.name.to_string(),
            BoardTile::Railroad(property) => property.name.to_string(),
            BoardTile::Utility(property) => property.name.to_string(),
            BoardTile::Event(tile) => tile.name.to_string(),
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

    /* Actions players can do in the game */
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

    pub fn pay_rent(&mut self, tenent: usize, landlord: usize, rent: i64) {
        self.players[tenent].pay(rent);
        self.players[landlord].collect(rent);
    }

    pub fn record_tile_ownership(&mut self, owner: usize, set_name: &String, tile: usize) {
        self.ownership_records[owner]
            .entry(set_name.to_string())
            .or_insert(HashSet::from([tile]))
            .insert(tile);
    }

    pub fn remove_tile_ownership(&mut self, owner: usize, set_name: &String, tile: usize) {
        if let Some(tiles) = self.ownership_records[owner].get_mut(set_name) {
            tiles.remove(&tile);
        }
    }

    pub fn is_set_complete(&self, player: usize, tile: usize) -> bool {
        //! If the given player owns every single tile of the colour set described
        //! by the given tile's set, the set is considered complete
        let set_name: &String = self.get_set_name(tile);

        if let (Some(player_set), Some(property_set)) = (
            self.ownership_records[player].get(set_name),
            self.property_sets.get(set_name),
        ) {
            player_set.len() == property_set.len()
        } else {
            false
        }
    }

    pub fn update_rent_based_on_set_completion(&mut self, owner: usize, tile: usize) {
        // Assumes the tile ownership is already recorded because it bases info off of it
        match self.board.get_tile(tile) {
            BoardTile::Street(_) => {
                if self.is_set_complete(owner, tile) {
                    let owned_set = &self.ownership_records[owner][self.get_set_name(tile)];
                    for street_tile in owned_set {
                        let tile_in_set = self.board.get_tile_mut(*street_tile);
                        if let BoardTile::Street(t) = tile_in_set {
                            t.update_rent_full_set();
                        }
                    }
                }
            }
            BoardTile::Railroad(_) => {
                let owned_railroads: &HashSet<usize> =
                    &self.ownership_records[owner][self.get_set_name(tile)];
                for railroad_tile in owned_railroads {
                    let tile_in_set: &mut BoardTile = self.board.get_tile_mut(*railroad_tile);
                    if let BoardTile::Railroad(t) = tile_in_set {
                        t.update_rent_total_number_of_owned_railroads(owned_railroads.len());
                    }
                }
            }
            BoardTile::Utility(_) => {
                let owned_utilities: &HashSet<usize> =
                    &self.ownership_records[owner][self.get_set_name(tile)];
                for utility_tile in owned_utilities {
                    let tile_in_set: &mut BoardTile = self.board.get_tile_mut(*utility_tile);
                    if let BoardTile::Utility(t) = tile_in_set {
                        t.update_rent_total_number_of_owned_utilities(owned_utilities.len());
                    }
                }
            }
            BoardTile::Event(_) => panic!("No rent to update!"),
        }
    }

    pub fn buy_tile(&mut self, buyer: usize, tile: usize) -> Option<usize> {
        let set_name: String = self.get_set_name(tile).to_string();

        match self.board.get_tile_mut(tile) {
            BoardTile::Street(property) if property.owner.is_none() => {
                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &self.players[buyer].colour);
                self.players[buyer].pay(property.property_cost);
                self.record_tile_ownership(buyer, &set_name, tile);

                // Check if player completes the set to apply double rent rule on full set ownership
                self.update_rent_based_on_set_completion(buyer, tile);

                Some(tile)
            }
            BoardTile::Railroad(property) if property.owner.is_none() => {
                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &self.players[buyer].colour);
                self.players[buyer].pay(property.property_cost);
                self.record_tile_ownership(buyer, &set_name, tile);

                // Rent scales for all tiles in the set based on the number of it owned by the buyer
                self.update_rent_based_on_set_completion(buyer, tile);

                Some(tile)
            }
            BoardTile::Utility(property) if property.owner.is_none() => {
                // Transfer ownership of the tile to the buyer
                property.acquired_by(buyer, &self.players[buyer].colour);
                self.players[buyer].pay(property.property_cost);
                self.record_tile_ownership(buyer, &set_name, tile);

                // Rent scales for all tiles in the set based on the number of it owned by the buyer
                self.update_rent_based_on_set_completion(buyer, tile);

                Some(tile)
            }
            _ => None, // EventTiles cannot be purchased
        }
    }

    pub fn trade_start(&mut self, proposer: usize, receiver: usize) {
        self.proposer = proposer;
        self.receiver = receiver;
    }

    pub fn is_tile_owned_by_player(&self, player: usize, tile: usize) -> bool {
        let mut is_owned: bool = false;
        for (_, tiles) in &self.ownership_records[player] {
            if tiles.contains(&tile) {
                is_owned = true;
                break;
            }
        }

        is_owned
    }

    pub fn trade_give_tile(&mut self, tile: usize) -> bool {
        if self.is_tile_owned_by_player(self.proposer, tile) {
            self.proposer_tiles.insert(tile, self.get_tile_name(tile));
            true
        } else {
            false
        }
    }

    pub fn trade_give_money(&mut self, money: i64) -> bool {
        if self.players[self.proposer].money >= money {
            self.proposer_money = money;
            true
        } else {
            false
        }
    }

    pub fn trade_take_tile(&mut self, tile: usize) -> bool {
        if self.is_tile_owned_by_player(self.receiver, tile) {
            self.receiver_tiles.insert(tile, self.get_tile_name(tile));
            true
        } else {
            false
        }
    }

    pub fn trade_take_money(&mut self, money: i64) -> bool {
        if self.players[self.receiver].money >= money {
            self.receiver_money = money;
            true
        } else {
            false
        }
    }

    pub fn trade_mine_pop(&mut self, item_id: i64) -> bool {
        // Returns true if removal from offer is successfully removed
        // Returns false if it was never there in the first place
        if item_id == 100 && self.proposer_money != 0 {
            self.proposer_money = 0;
            true
        } else if let Some(_) = self.proposer_tiles.remove(&(item_id as usize)) {
            true
        } else {
            false
        }
    }

    pub fn trade_their_pop(&mut self, item_id: i64) -> bool {
        // Returns true if removal from offer is successfully removed
        // Returns false if it was never there in the first place
        if item_id == 100 && self.receiver_money != 0 {
            self.receiver_money = 0;
            true
        } else if let Some(_) = self.receiver_tiles.remove(&(item_id as usize)) {
            true
        } else {
            false
        }
    }

    pub fn accept_trade(&mut self) {
        // Trade money
        self.players[self.proposer].pay(self.proposer_money);
        self.players[self.proposer].collect(self.receiver_money);

        self.players[self.receiver].pay(self.receiver_money);
        self.players[self.receiver].collect(self.proposer_money);

        // Let the proposer get the tiles traded by the receiver
        let tile_ids: Vec<usize> = self.receiver_tiles.keys().cloned().collect();
        for tile_id in tile_ids {
            match self.board.get_tile_mut(tile_id) {
                BoardTile::Street(tile) => {
                    tile.acquired_by(self.proposer, &self.players[self.proposer].colour)
                }
                BoardTile::Railroad(tile) => {
                    tile.acquired_by(self.proposer, &self.players[self.proposer].colour)
                }
                BoardTile::Utility(tile) => {
                    tile.acquired_by(self.proposer, &self.players[self.proposer].colour)
                }
                BoardTile::Event(_) => panic!("Cannot trade Event tiles"),
            }

            let set_name = self.get_set_name(tile_id).to_string();
            self.record_tile_ownership(self.proposer, &set_name, tile_id);
            self.remove_tile_ownership(self.receiver, &set_name, tile_id);
            self.update_rent_based_on_set_completion(self.proposer, tile_id);
        }

        // Let the receiver get the tiles traded by the proposer
        let tile_ids: Vec<usize> = self.proposer_tiles.keys().cloned().collect();
        for tile_id in tile_ids {
            match self.board.get_tile_mut(tile_id) {
                BoardTile::Street(tile) => {
                    tile.acquired_by(self.receiver, &self.players[self.receiver].colour)
                }
                BoardTile::Railroad(tile) => {
                    tile.acquired_by(self.receiver, &self.players[self.receiver].colour)
                }
                BoardTile::Utility(tile) => {
                    tile.acquired_by(self.receiver, &self.players[self.receiver].colour)
                }
                BoardTile::Event(_) => panic!("Cannot trade Event tiles"),
            }

            let set_name = self.get_set_name(tile_id).to_string();
            self.record_tile_ownership(self.receiver, &set_name, tile_id);
            self.remove_tile_ownership(self.proposer, &set_name, tile_id);
            self.update_rent_based_on_set_completion(self.receiver, tile_id);
        }
        self.clear_trade();
    }

    pub fn clear_trade(&mut self) {
        self.proposer = 0;
        self.proposer_money = 0;
        self.proposer_tiles.clear();
        self.receiver = 0;
        self.receiver_money = 0;
        self.receiver_tiles.clear();
    }

    pub fn reject_trade(&mut self) {
        self.clear_trade();
    }
}
