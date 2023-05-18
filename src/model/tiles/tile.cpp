#include "src/model/tiles/tile.hpp"

Tile::Tile(const json &tile_data, int id)
    : id{ id }, name{ tile_data["name"] }, group{ tile_data["group"] } {}

void Tile::set_owner(Player &player) {
  owner_id = player.get_id();
  owner_marker = player.get_piece();
}