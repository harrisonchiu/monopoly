#include "tiles/corner.hpp"
#include "tiles/tile.hpp"

#include "board.hpp"
#include "utils/color.hpp"

#include <string>

Corner::Corner(const json &tile_data, int id) : Event(tile_data, id) { update_detail(); }

void Corner::update_detail() { Tile::set_detail(Color::empty(Board::get_tile_length())); }