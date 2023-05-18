#include "src/model/tiles/tile.hpp"

#include <limits>
#include <string>

Tile::Tile(const json &tile_data, int id)
    : id{ id },
      name{ tile_data["name"].get<std::string>() },
      group{ tile_data["group"].get<std::string>() },
      color{ fmt::format(Color::get(group), "▔▔▔▔▔▔▔") },
      cost{ tile_data.value("cost", std::numeric_limits<int>::max()) },
      is_ownable{ tile_data.contains("cost") } {}

void Tile::set_owner(const Player &player) {
  owner_id = player.get_id();
  owner_marker = player.get_piece();
}