#include "src/model/tiles/tile.hpp"

#include "src/model/players/player.hpp"
#include "src/utils/color.hpp"

#include <fmt/core.h>

#include <string>

Tile::Tile(const json &tile_data, int id)
    : id{ id },
      name{ tile_data["name"].get<std::string>() },
      group{ tile_data["group"].get<std::string>() },
      color{ Color::get(group) },
      box{ fmt::format(color, "▔▔▔▔▔▔▔") },
      cost{ tile_data.value("cost", maximum_cost) },
      is_ownable{ tile_data.contains("cost") } {}

void Tile::set_owner(const Player &player) { owner = player.get_token(); }